
## 2026-10-04-host-portable-tauri-ipc-tests — the custom-protocol origin measurement scoped to its host
**Section:** §Infrastructure Patterns → Build system (the 2026-09-01 `custom-protocol` measurement parenthetical)
**Change:** The parenthetical read "the same build with `--features tauri/custom-protocol` opened on `http://tauri.localhost/`" with no host named. It now adds that the measurement was taken on the Windows host and that the custom-protocol origin is per-OS: `tauri` 2.11.3's `tauri_protocol_url` is `http://tauri.localhost` on Windows/Android and `tauri://localhost` on Linux/macOS, as measured on the Linux dev host at this chunk. The Windows value stays.
**Why:** The chunk measured the per-OS rule on the Linux dev host, where the mock-runtime IPC tests' Windows literal resolved as a remote origin and six tests failed `not allowed. Plugin not found`. The plan listed this as an expected amendment, governed by the seed rule "Accurate this-chunk addition". Registries are unaffected (the edit is in §Infrastructure Patterns): §Established Decisions 37991 B, §Occupied Resources 38083 B.
**Ref:** .andromeda/runs/2026-10-04T01-18-54-wrap/
