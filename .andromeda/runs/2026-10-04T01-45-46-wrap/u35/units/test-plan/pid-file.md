### PID file

- **Location:** N/A — no daemon (security: "no inbound listener of its own"; arch: `conductor-cli` is a one-shot `#[tokio::main(flavor="current_thread")]` binary). Nothing to write or signal.
- **Lifecycle:** N/A — `cleanup` removes on-disk run artifacts and `runs.db` rows instead of signaling a long-lived process.
