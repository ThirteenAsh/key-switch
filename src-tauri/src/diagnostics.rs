use regex::Regex;
use std::{
    fs::{self, OpenOptions},
    io::{self, Write},
    path::{Path, PathBuf},
    sync::{Mutex, OnceLock},
};
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

pub(crate) const LOG_FILE_NAME: &str = "key-switch.log";
pub(crate) const LOG_BACKUP_FILE_NAME: &str = "key-switch.log.1";
pub(crate) const MAX_DETAIL_CHARS: usize = 2400;
pub(crate) static LOG_LOCK: Mutex<()> = Mutex::new(());
const MAX_FILE_SIZE: u64 = 1024 * 1024;
const BACKUP_COUNT: usize = 3;

struct Logger {
    directory: PathBuf,
    session: String,
}
static LOGGER: OnceLock<Logger> = OnceLock::new();

pub(crate) fn sanitize(detail: &str) -> String {
    static PATTERNS: OnceLock<[Regex; 4]> = OnceLock::new();
    let patterns = PATTERNS.get_or_init(|| [
        Regex::new(r#"(?i)\b(?:proxy-authorization|authorization|api[-_ ]?key|access[-_ ]?token|refresh[-_ ]?token|secret|password)\b["']?\s*[:=]\s*(?:"[^"]*"|'[^']*'|(?:Bearer|Basic)\s+[^\s,;]+|[^\s,;]+)"#).unwrap(),
        Regex::new(r"(?i)\b(?:Bearer|Basic)\s+[A-Za-z0-9._~+/=\-]+").unwrap(),
        Regex::new(r#"(?i)(?:https?|socks5h?)://[^\s"'<>]+"#).unwrap(),
        Regex::new(r"\bsk-[A-Za-z0-9_\-]{8,}").unwrap(),
    ]);
    let mut value = patterns[0]
        .replace_all(detail, "credential=[REDACTED]")
        .into_owned();
    value = patterns[1]
        .replace_all(&value, "credential=[REDACTED]")
        .into_owned();
    value = patterns[2]
        .replace_all(&value, |captures: &regex::Captures<'_>| {
            reqwest::Url::parse(&captures[0])
                .map(|url| format!("{}/[REDACTED]", url.origin().ascii_serialization()))
                .unwrap_or_else(|_| "[REDACTED_URL]".into())
        })
        .into_owned();
    value = patterns[3].replace_all(&value, "[REDACTED]").into_owned();
    value
        .chars()
        .filter(|ch| !ch.is_control() || *ch == '\t')
        .take(MAX_DETAIL_CHARS)
        .collect()
}

fn entry(level: &str, event: &str, detail: &str) -> String {
    let timestamp = OffsetDateTime::now_utc()
        .format(&Rfc3339)
        .unwrap_or_default();
    let session = LOGGER
        .get()
        .map(|logger| logger.session.as_str())
        .unwrap_or("startup");
    format!(
        "{timestamp} [{level}] {event} session={session} pid={} {}\n",
        std::process::id(),
        sanitize(detail)
    )
}

fn write_entry(directory: &Path, line: &str) -> io::Result<()> {
    fs::create_dir_all(directory)?;
    let file = directory.join(LOG_FILE_NAME);
    if file.metadata().map(|metadata| metadata.len()).unwrap_or(0) >= MAX_FILE_SIZE {
        let oldest = directory.join(format!("{LOG_FILE_NAME}.{BACKUP_COUNT}"));
        if oldest.exists() {
            fs::remove_file(oldest)?;
        }
        for number in (1..BACKUP_COUNT).rev() {
            let previous = directory.join(format!("{LOG_FILE_NAME}.{number}"));
            if previous.exists() {
                fs::rename(
                    previous,
                    directory.join(format!("{LOG_FILE_NAME}.{}", number + 1)),
                )?;
            }
        }
        fs::rename(&file, directory.join(LOG_BACKUP_FILE_NAME))?;
    }
    let mut output = OpenOptions::new().create(true).append(true).open(file)?;
    output.write_all(line.as_bytes())?;
    output.flush()?;
    output.sync_data()
}

pub(crate) fn write(
    directory: &Path,
    level: &str,
    event: &str,
    detail: &str,
) -> Result<(), String> {
    let line = entry(level, event, detail);
    let result = LOG_LOCK
        .lock()
        .map_err(|_| "日志写入锁不可用".to_string())
        .and_then(|_guard| {
            write_entry(directory, &line).map_err(|error| format!("无法写入日志：{error}"))
        });
    if result.is_err() {
        eprintln!("logging_fallback {line}");
    }
    result
}

pub(crate) fn record(level: &str, event: &str, detail: &str) {
    if let Some(logger) = LOGGER.get() {
        let _ = write(&logger.directory, level, event, detail);
    } else {
        eprint!("{}", entry(level, event, detail));
    }
}

pub(crate) fn init(directory: PathBuf, version: &str) {
    let _ = LOGGER.set(Logger {
        directory,
        session: format!("{}-{}", std::process::id(), crate::now()),
    });
    record(
        "INFO",
        "application_started",
        &format!(
            "version={version} os={} arch={}",
            std::env::consts::OS,
            std::env::consts::ARCH
        ),
    );
}

pub(crate) fn install_panic_hook() {
    std::panic::set_hook(Box::new(|info| {
        // Panic payloads may contain user data. Record the location only, and never block on a logger lock.
        let detail = info
            .location()
            .map(|location| {
                format!(
                    "file={} line={} column={}",
                    location.file(),
                    location.line(),
                    location.column()
                )
            })
            .unwrap_or_default();
        let line = entry("ERROR", "native_panic", &detail);
        if let (Some(logger), Ok(_guard)) = (LOGGER.get(), LOG_LOCK.try_lock()) {
            if write_entry(&logger.directory, &line).is_ok() {
                return;
            }
        }
        eprint!("{line}");
    }));
}

pub(crate) fn clear(directory: &Path) -> Result<(), String> {
    let _guard = LOG_LOCK
        .lock()
        .map_err(|_| "日志写入锁不可用".to_string())?;
    for number in 0..=BACKUP_COUNT {
        let file = directory.join(if number == 0 {
            LOG_FILE_NAME.into()
        } else {
            format!("{LOG_FILE_NAME}.{number}")
        });
        if file.exists() {
            fs::remove_file(file).map_err(|error| format!("无法清空日志：{error}"))?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn failed_log_write_returns_error_without_recursing() {
        let path = std::env::temp_dir().join(format!(
            "key-switch-log-failure-test-{}-{}",
            std::process::id(),
            crate::now()
        ));
        fs::write(&path, b"not a directory").unwrap();
        assert!(write(&path, "ERROR", "test", "reason=write_failure").is_err());
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn redacts_credentials_and_entire_url_paths_queries() {
        let secret = "example-sensitive-value";
        for value in [
            format!("Authorization: Bearer {secret}"),
            format!("Bearer {secret}"),
            format!("\"password\": \"{secret}\""),
            format!("https://user:{secret}@example.com/{secret}?token={secret}#fragment"),
        ] {
            assert!(!sanitize(&value).contains(secret));
        }
        assert_eq!(sanitize("first\nsecond\rthird"), "firstsecondthird");
        assert_eq!(
            sanitize(&"字".repeat(3000)).chars().count(),
            MAX_DETAIL_CHARS
        );
    }

    #[test]
    fn rotates_logs_and_clears_all_backups() {
        let directory = std::env::temp_dir().join(format!(
            "key-switch-log-test-{}-{}",
            std::process::id(),
            crate::now()
        ));
        fs::create_dir_all(&directory).unwrap();
        for number in 0..4 {
            fs::write(
                directory.join(LOG_FILE_NAME),
                vec![b'x'; MAX_FILE_SIZE as usize],
            )
            .unwrap();
            write(&directory, "INFO", "test", &format!("round={number}")).unwrap();
        }
        assert!(directory.join("key-switch.log.3").exists());
        assert!(!directory.join("key-switch.log.4").exists());
        clear(&directory).unwrap();
        assert_eq!(fs::read_dir(&directory).unwrap().count(), 0);
        fs::remove_dir(&directory).unwrap();
    }
}
