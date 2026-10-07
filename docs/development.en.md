# Key Switch Development Guide

[简体中文](./development.md) · English · [繁體中文](./development.zh-TW.md)

## 1. Prerequisites

- Node.js 22 LTS or a later compatible version
- npm
- The Rust stable toolchain
- Platform dependencies required by Tauri 2

Windows also requires Microsoft C++ Build Tools and WebView2 Runtime.

## 2. Run the project

From the project root:

```bash
npm install
npm run tauri:dev
```

To work on the frontend only:

```bash
npm run dev
```

Browser mode does not provide Tauri capabilities such as the system credential store, clipboard, application data directory, or persistent application settings. Settings changed in browser mode only last for that session.

## 3. Common checks

```bash
# Type-check and build the frontend for production
npm run build

# Frontend navigation, update interaction, and logging tests
npm run test:frontend

# Format, compile, and test Rust
cd src-tauri
cargo fmt --all -- --check
cargo check --locked
cargo test --locked
cargo clippy --all-targets --all-features --locked -- -D warnings
```

Changes to frontend/backend contracts, the settings file, or security-sensitive logic must pass both the frontend build and Rust tests before being submitted.

### Versioning and releases

The current application version is `1.1.1`. Version changes must update `package.json`, root package versions in `package-lock.json`, `src-tauri/Cargo.toml`, the application entry in `src-tauri/Cargo.lock`, `src-tauri/tauri.conf.json` including the Windows WiX version, all four README badges, and the default release workflow tag.

UI fallback versions come from `package.json`; Rust request headers use `CARGO_PKG_VERSION` instead of separate version strings. The running desktop version still comes from Tauri.

CI and release builds use Node.js 22 and run `npm run test:frontend`. The release tag must match the application version. `.github/release-notes.md` contains the current release notes, and the release workflow creates drafts only. Commit, push, tag, and publish actions require explicit release instructions.

## 4. Architecture and directory responsibilities

| Directory | Responsibility |
| --- | --- |
| `src/views/` | Main page, provider management, and categorized settings pages |
| `src/components/` | Reusable Vue components, dialogs, and base UI components |
| `src/api/` | TypeScript types and wrappers for Tauri commands |
| `src/stores/` | Pinia state, business operations, and the settings persistence queue |
| `src/i18n/` | Locale resolution, Vue I18n setup, error translation, and four message sets |
| `src/data/` | Non-sensitive static data such as the built-in provider catalog |
| `src-tauri/src/` | Rust commands, local files, credential storage, network checks, and updates |
| `src-tauri/capabilities/` | Least-privilege Tauri capability declarations |
| `docs/` | Development documentation and README assets |

The frontend owns presentation, interaction, and non-sensitive state. Sensitive data, persistent files, external requests, and operating-system capabilities belong on the Rust side. New frontend code should keep the Vue `<script setup lang="ts">` style and complete type definitions.

### Page navigation and ordering

- The sidebar sizes itself using a hidden width guide for built-in Chinese and English names plus CSS inline-size containment; custom long names do not expand it. Both headings share a height. Provider type and platform URL follow the right heading; long URLs truncate with a full tooltip and remain clickable.
- Provider changes use short fades for the heading and Key content, with a slight content shift. Reduced motion disables fades and shifts. Leaving content ignores pointer interaction. The actions column reserves a fixed width for all four buttons, and masked cells keep table layout.
- The main page lists providers on the left and the selected provider's Keys on the right. Provider management and settings links sit at the bottom left.
- Provider selection lives only in the current Pinia session, without new data files or settings fields. The sidebar has no provider search and always shows the full list. The first provider is the default and the fallback when the selected provider is deleted. Key search terms live only in the page component.
- Sidebar ordering reuses pointer capture, the 4px activation threshold, and FLIP drop animations. The drag handle also supports `Alt + ↑ / ↓`; reduced motion skips drop animations.
- Ordering uses the existing `reorder_providers` command with the complete ID list. Further reordering is disabled while saving; failures restore the previous order and display a translated error.
- `/settings` redirects to `/settings/general`. Existing settings are grouped into General (language, appearance), Data & logs (storage directory, logs), and About (version, repository, updates), at `/settings/general`, `/settings/data`, and `/settings/about`. Back returns to the main page.
- Version and update installation status appear at the bottom of the sidebar. Settings continue using the existing Store queue and unchanged settings/business data files. Update commands add a connection mode and operation ID scoped to the current operation.
- Settings category pages share the main page heading height, text and icon sizes, with compact controls. Bottom sidebar navigation wraps when space is limited and preserves full labels in every language.

## 5. Local data and application settings

Runtime data is stored in Tauri's per-user application data directory:

| Content | Location | Notes |
| --- | --- | --- |
| Provider and key metadata | `key-switch-data.json` | Never contains complete API Keys |
| Application settings | `settings.json` | Non-sensitive preferences only |
| Complete API Keys | System credential store | Never written to JSON |
| Application logs | `logs/` | Must not contain complete keys or credentials |

Current settings file example:

```json
{
  "schemaVersion": 1,
  "localePreference": "system",
  "themePreference": "system"
}
```

`themePreference` accepts `system`, `light`, or `dark`. The theme is selected on the Settings page; `system` follows operating-system appearance changes in real time. The theme is applied to both WebView content and the native Tauri window title bar, and changes are persisted through the settings Store write queue.

Settings startup flow:

1. `src/main.ts` loads the settings Store before mounting Vue.
2. Rust reads and validates `settings.json`, creating defaults on the first run.
3. For upgraded users, the old `key-switch.locale` value is migrated only when the settings file is first created.
4. The legacy `localStorage` key is removed after a successful migration.
5. The language and theme are applied before Vue mounts, preventing a startup flash.

Settings writes are protected as follows:

- A frontend queue preserves the order of rapid consecutive changes.
- Rust's `SETTINGS_LOCK` prevents concurrent file writes.
- New content is written to `settings.json.tmp` and explicitly flushed to disk.
- The current file is renamed to `settings.json.bak` before the temporary file replaces it.
- On failure, Rust restores the previous file and the frontend rolls back to the most recently saved settings.
- On startup after an interrupted replacement, a fully written temporary file is preferred; otherwise the backup is restored.

When adding a setting, update all of the following:

1. Rust `AppSettings` or the relevant nested settings type.
2. Rust `Default` values, range validation, and any required migration.
3. TypeScript `AppSettings`.
4. `normalizeSettings()` in `src/stores/settings.ts`.
5. The settings UI, all four message sets, and relevant tests.

The frontend merges partial changes with `settingsStore.updateSettings()`, but every command and file write contains the complete settings object. A compatible field with a default can keep the current `schemaVersion`. Removing a field, changing its type, or changing its meaning requires a version bump and an explicit migration. Do not change only the version number: unsupported versions are intentionally rejected.

Logging behavior: native events are written to `logs/key-switch.log`. Each file rotates at 1 MB with 3 backups, all removed by Clear logs. New entries use RFC 3339 timestamps with session and process IDs; startup includes version, OS and architecture. Native command errors are logged after redaction, while the UI receives stable error codes only. Update events include operation ID, stage, proxy source, elapsed time, byte counts and signature outcomes. Key checks record non-sensitive IDs, HTTP status and failure categories, including early DNS failures. Never log full keys, credentials, URL paths/queries or response bodies.

Frontend logging covers Vue errors, unhandled rejections and resource failures using capture listeners. The native logging boundary redacts and truncates all details. Failed writes fall back to redacted stderr entries; frontend write failures emit a single warning without details. Rust panic logging records locations only, without potentially sensitive payloads. Forced process termination does not guarantee a final entry.

### Update connections and direct retry

- Release checks, manifests and downloads default to environment and supported system proxies, honoring `NO_PROXY`; environment variables take priority. Windows manual proxies are supported; PAC configuration is outside this implementation.
- Proxied network, connection, timeout or HTTP request failures prompt during manual operations. Startup checks remain quiet and log failures. Invalid update data or signatures never trigger direct retry prompts.
- Consent retries once using `.no_proxy()`. The mode and operation ID carry from checking through this installation; cancellation makes no further requests. The next check starts with the default proxy policy. No settings or permanent preference are added. Application-level direct mode does not bypass TUN routing.
- Checks have a 12-second limit, manifests 15 seconds, downloads 300 seconds, and connections 5 seconds. Download size is capped at 256 MB. The previous request ends before retrying; failed direct retries use existing errors.
- The confirmation dialog uses theme variables and short transitions, with four languages, Escape/backdrop cancellation, Tab focus containment, focus restoration and reduced-motion support.

## 6. Internationalization rules

Supported preferences:

- `zh-CN`: Simplified Chinese
- `zh-TW`: Traditional Chinese
- `en-US`: English
- `ja-JP`: Japanese
- `system`: Follow the operating system

If the system locale cannot be read, the app falls back to Simplified Chinese. A readable but unsupported non-Chinese system locale falls back to English.

Development rules:

- Every user-visible string must use Vue I18n, including buttons, toasts, errors, placeholders, `title`, `aria-label`, and image `alt` text.
- `src/i18n/messages/zh-CN.ts` defines the message shape; every other locale must satisfy the same `MessageSchema`.
- Add or remove keys in all four message sets and run `npm run build` to verify their structure.
- Use semantic, domain-based keys instead of source-language text as keys.
- Custom provider names and names already stored in user data are user content: never translate or silently rename them.
- When a built-in provider is added, Chinese locales use its Chinese catalog name. English, Japanese, and other non-Chinese locales use its English name. The displayed name is persisted at creation time.
- The provider catalog maintains stable IDs plus Chinese and English names; changing the locale must not rewrite stored records.
- Animated components must support `prefers-reduced-motion`. Custom controls such as selects must retain keyboard behavior and accessibility semantics.

## 7. Tauri commands and error handling

- Call `invoke` through `src/api/app.ts`; do not scatter command strings across page components.
- Define native commands with `#[tauri::command]` and register them in `tauri::generate_handler!` in `src-tauri/src/lib.rs`.
- Keep command arguments and return types synchronized between Rust and TypeScript.
- Rust exposes only stable, non-sensitive error codes to the UI; raw internal errors must not be displayed to users.
- `src/i18n/errors.ts` maps error codes to translated messages.
- Business logic must never determine state by matching Chinese or English error text.
- A new error code requires a Rust mapping, a frontend mapping, all four translations, and tests.

## 8. Security rules

- Complete API Keys may only be read and handled by Rust. They must never enter logs, URLs, persistent DOM state, frontend Stores, `localStorage`, or `settings.json`.
- List commands return masked values and non-sensitive metadata only. Secrets are stored in the system credential store.
- Copying, decrypting, and validation happen on the Rust side, with plaintext lifetimes kept as short as practical.
- External validation may only contact explicitly allowed provider endpoints and must enforce timeouts, response-size limits, and clear error categories.
- The settings file stores non-sensitive preferences only. Review every new settings field for sensitivity.
- Review `src-tauri/capabilities/` before adding a Tauri plugin or operating-system capability, and keep permissions minimal.
- Never commit real API Keys in source, fixtures, Issues, pull requests, logs, or screenshots.

### Custom provider key checks

- Custom providers default to `none`. Existing records without a check configuration must also fall back to `none` and the unsupported status.
- `platformUrl` is only for opening the management console and must not be used as a check endpoint. Check configuration is stored separately in provider metadata and currently supports OpenAI-compatible, Bearer token, and API key header modes.
- A check configuration stores only an HTTPS endpoint, authentication mode, and a non-sensitive header name. The full key is still read temporarily from the system credential store by Rust only.
- Custom check URLs must not contain credentials, query parameters, or fragments. Loopback, private, link-local, reserved addresses, and hostnames resolving to them are rejected, and HTTP redirects remain disabled.
- Check requests do not read or log response bodies. Rate limits, timeouts, network failures, server failures, and endpoint errors must use distinct stable, non-sensitive result codes and must not be treated as invalid keys.
- Changing a provider check configuration must clear previous key check results. No network request may be made when checking is unsupported.

## 9. Change checklist

- UI changes: verify all four locales, long text, keyboard interaction, and reduced motion.
- Settings changes: verify defaults, validation, full-object merging, rollback, and old-file compatibility.
- Rust command changes: verify frontend/backend types, command registration, and structured error codes.
- Data model changes: never silently rewrite existing names, notes, or other user data.
- Security changes: confirm sensitive values cannot reach frontend persistence, settings files, or logs.
- Documentation changes: keep `development.md`, `development.en.md`, and `development.zh-TW.md` synchronized.
- Run `npm run build`, `npm run test:frontend`, `cargo fmt --all -- --check`, `cargo check --locked`, `cargo test --locked`, and the CI command `cargo clippy --all-targets --all-features --locked -- -D warnings`.
