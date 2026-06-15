//! `conductor` — headless agent-run CLI (source of truth + release gate).

fn main() {
    conductor_core::init_observability("conductor", None);
}
