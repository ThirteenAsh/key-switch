use crate::{diagnostics, CommandError};
use hyper_util::client::proxy::matcher::Matcher;
use reqwest::{ClientBuilder, Url};
use serde::{Deserialize, Serialize};
use std::{
    sync::atomic::{AtomicU64, Ordering},
    time::{Duration, Instant},
};

#[derive(Clone, Copy, Default, Deserialize, Serialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) enum NetworkMode {
    #[default]
    Auto,
    Direct,
}

pub(crate) struct UpdateRequest {
    pub(crate) mode: NetworkMode,
    pub(crate) id: String,
    started: Instant,
    system: Matcher,
    environment: Matcher,
}

impl UpdateRequest {
    pub(crate) fn new(mode: Option<NetworkMode>, id: Option<String>, stage: &str) -> Self {
        static NEXT_ID: AtomicU64 = AtomicU64::new(1);
        let id = id
            .filter(|id| {
                !id.is_empty()
                    && id.len() <= 64
                    && id.chars().all(|ch| ch.is_ascii_alphanumeric() || ch == '-')
            })
            .unwrap_or_else(|| {
                format!(
                    "{}-{}-{}",
                    std::process::id(),
                    crate::now(),
                    NEXT_ID.fetch_add(1, Ordering::Relaxed)
                )
            });
        let request = Self {
            mode: mode.unwrap_or_default(),
            id,
            started: Instant::now(),
            system: Matcher::from_system(),
            environment: Matcher::from_env(),
        };
        request.log("INFO", stage, "started", "");
        request
    }

    pub(crate) fn configure(&self, builder: ClientBuilder) -> ClientBuilder {
        let builder = builder
            .connect_timeout(Duration::from_secs(5))
            .redirect(reqwest::redirect::Policy::limited(5));
        if self.mode == NetworkMode::Direct {
            builder.no_proxy()
        } else {
            builder
        }
    }

    pub(crate) fn proxy_source(&self, url: &Url) -> &'static str {
        proxy_source(self.mode, &self.system, &self.environment, url)
    }

    pub(crate) fn log(&self, level: &str, stage: &str, outcome: &str, detail: &str) {
        diagnostics::record(
            level,
            "update_operation",
            &format!(
                "operation={} mode={:?} stage={stage} outcome={outcome} elapsed_ms={} {detail}",
                self.id,
                self.mode,
                self.started.elapsed().as_millis()
            ),
        );
    }

    pub(crate) fn error(
        &self,
        stage: &str,
        url: &Url,
        fallback_code: &'static str,
        retryable: bool,
        reason: &str,
    ) -> CommandError {
        let source = self.proxy_source(url);
        let code = failure_code(source != "none", retryable, fallback_code);
        self.log(
            "ERROR",
            stage,
            "failed",
            &format!(
                "code={code} proxy_source={source} host={} {reason}",
                url.host_str().unwrap_or("unknown")
            ),
        );
        CommandError { code }
    }

    pub(crate) fn network_error(
        &self,
        stage: &str,
        url: &Url,
        fallback_code: &'static str,
        error: &reqwest::Error,
    ) -> CommandError {
        let target = error.url().unwrap_or(url);
        let retryable = !error.is_decode()
            && !error.is_builder()
            && (error.is_timeout() || error.is_connect() || error.is_request() || error.is_body());
        self.error(
            stage,
            target,
            fallback_code,
            retryable,
            &format!(
                "kind={} status={} cause={error:?}",
                if error.is_timeout() {
                    "timeout"
                } else if error.is_connect() {
                    "connection"
                } else {
                    "request"
                },
                error
                    .status()
                    .map(|status| status.as_u16().to_string())
                    .unwrap_or_else(|| "none".into())
            ),
        )
    }

    pub(crate) fn updater_error(
        &self,
        stage: &str,
        url: &Url,
        fallback_code: &'static str,
        error: &tauri_plugin_updater::Error,
    ) -> CommandError {
        if let tauri_plugin_updater::Error::Reqwest(error) = error {
            return self.network_error(stage, url, fallback_code, error);
        }
        if matches!(
            error,
            tauri_plugin_updater::Error::Network(_) | tauri_plugin_updater::Error::ReleaseNotFound
        ) {
            return self.error(
                stage,
                url,
                fallback_code,
                true,
                &format!("kind=http_response cause={error:?}"),
            );
        }
        // Invalid metadata, signatures and installer errors must never offer a network fallback.
        self.error(
            stage,
            url,
            fallback_code,
            false,
            &format!("cause={error:?}"),
        )
    }
}

fn proxy_source(
    mode: NetworkMode,
    system: &Matcher,
    environment: &Matcher,
    url: &Url,
) -> &'static str {
    if mode == NetworkMode::Direct {
        return "none";
    }
    let Ok(uri) = url.as_str().parse() else {
        return "none";
    };
    if system.intercept(&uri).is_none() {
        "none"
    } else if environment.intercept(&uri).is_some() {
        "environment"
    } else {
        "system"
    }
}

fn failure_code(proxied: bool, retryable: bool, fallback: &'static str) -> &'static str {
    if proxied && retryable {
        "UPDATE_PROXY_FAILED"
    } else {
        fallback
    }
}

pub(crate) const MANIFEST_TIMEOUT: Duration = Duration::from_secs(15);
pub(crate) const DOWNLOAD_TIMEOUT: Duration = Duration::from_secs(300);

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test(flavor = "current_thread")]
    async fn failed_proxy_is_reported_and_direct_retry_bypasses_it() {
        use std::{
            io::{Read, Write},
            net::TcpListener,
        };
        let origin = TcpListener::bind("127.0.0.1:0").unwrap();
        let url = Url::parse(&format!("http://{}/update", origin.local_addr().unwrap())).unwrap();
        let closed_proxy = TcpListener::bind("127.0.0.1:0").unwrap();
        let proxy_url = format!("http://{}", closed_proxy.local_addr().unwrap());
        drop(closed_proxy);
        let server = std::thread::spawn(move || {
            let (mut socket, _) = origin.accept().unwrap();
            socket
                .set_read_timeout(Some(Duration::from_secs(5)))
                .unwrap();
            let mut buffer = [0; 4096];
            let received = socket.read(&mut buffer).unwrap();
            assert!(received > 0, "the direct request must reach the origin");
            socket
                .write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\nConnection: close\r\n\r\nok")
                .unwrap();
        });
        let mut request = UpdateRequest::new(Some(NetworkMode::Auto), None, "check");
        request.system = Matcher::builder().all(&proxy_url).build();
        request.environment = Matcher::builder().all(&proxy_url).build();
        let builder = || {
            reqwest::Client::builder()
                .proxy(reqwest::Proxy::all(&proxy_url).unwrap())
                .timeout(Duration::from_secs(5))
        };
        let error = request
            .configure(builder())
            .build()
            .unwrap()
            .get(url.clone())
            .send()
            .await
            .unwrap_err();
        assert_eq!(
            request
                .network_error("check", &url, "UPDATE_CHECK_FAILED", &error)
                .code,
            "UPDATE_PROXY_FAILED"
        );
        request.mode = NetworkMode::Direct;
        let response = request
            .configure(builder())
            .build()
            .unwrap()
            .get(url.clone())
            .send()
            .await
            .unwrap();
        assert_eq!(response.text().await.unwrap(), "ok");
        assert_eq!(request.proxy_source(&url), "none");
        server.join().unwrap();
    }

    #[test]
    fn only_retryable_proxied_failures_offer_direct_retry() {
        assert_eq!(
            failure_code(true, true, "UPDATE_DOWNLOAD_TIMEOUT"),
            "UPDATE_PROXY_FAILED"
        );
        assert_eq!(
            failure_code(false, true, "UPDATE_DOWNLOAD_TIMEOUT"),
            "UPDATE_DOWNLOAD_TIMEOUT"
        );
        assert_eq!(
            failure_code(true, false, "UPDATE_DATA_INVALID"),
            "UPDATE_DATA_INVALID"
        );
        assert_eq!(
            failure_code(true, false, "UPDATE_DOWNLOAD_FAILED"),
            "UPDATE_DOWNLOAD_FAILED"
        );
    }

    #[test]
    fn http_failures_can_retry_but_invalid_metadata_and_signatures_cannot() {
        let url = Url::parse("https://github.com/example/latest.json").unwrap();
        let mut request = UpdateRequest::new(Some(NetworkMode::Auto), None, "manifest");
        request.system = Matcher::builder().all("http://127.0.0.1:7890").build();
        assert_eq!(
            request
                .updater_error(
                    "manifest",
                    &url,
                    "UPDATE_MANIFEST_FAILED",
                    &tauri_plugin_updater::Error::ReleaseNotFound
                )
                .code,
            "UPDATE_PROXY_FAILED"
        );
        let invalid_json = serde_json::from_str::<serde_json::Value>("invalid").unwrap_err();
        assert_eq!(
            request
                .updater_error(
                    "manifest",
                    &url,
                    "UPDATE_MANIFEST_FAILED",
                    &tauri_plugin_updater::Error::Serialization(invalid_json)
                )
                .code,
            "UPDATE_MANIFEST_FAILED"
        );
        assert_eq!(
            request
                .updater_error(
                    "download",
                    &url,
                    "UPDATE_DOWNLOAD_FAILED",
                    &tauri_plugin_updater::Error::SignatureUtf8("invalid signature".into())
                )
                .code,
            "UPDATE_DOWNLOAD_FAILED"
        );
        request.mode = NetworkMode::Direct;
        assert_eq!(
            request
                .updater_error(
                    "manifest",
                    &url,
                    "UPDATE_MANIFEST_FAILED",
                    &tauri_plugin_updater::Error::ReleaseNotFound
                )
                .code,
            "UPDATE_MANIFEST_FAILED"
        );
    }

    #[test]
    fn direct_mode_and_no_proxy_exclusions_do_not_offer_proxy_retry() {
        let proxy = Matcher::builder().all("http://127.0.0.1:7890").build();
        let empty = Matcher::builder().build();
        let url = Url::parse("https://github.com/example").unwrap();
        assert_eq!(
            proxy_source(NetworkMode::Auto, &proxy, &empty, &url),
            "system"
        );
        assert_eq!(
            proxy_source(NetworkMode::Auto, &proxy, &proxy, &url),
            "environment"
        );
        assert_eq!(
            proxy_source(NetworkMode::Direct, &proxy, &proxy, &url),
            "none"
        );
        let bypass = Matcher::builder()
            .all("http://127.0.0.1:7890")
            .no("github.com")
            .build();
        assert_eq!(
            proxy_source(NetworkMode::Auto, &bypass, &bypass, &url),
            "none"
        );
    }
}
