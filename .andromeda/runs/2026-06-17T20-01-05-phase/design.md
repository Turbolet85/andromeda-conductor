# design extract

## No domain coverage
Out-of-scope — the determinism-replay harness is a backend test chunk (`insta` golden snapshots + `proptest` properties) within `conductor-timeline`, testing the scheduler's `Vec<PhaseTransition>` stream shape under a virtual clock. Design-system coverage applies only to rendered surfaces (desktop-webview, cli) — no new UI surface, tokens, component patterns, animation, or accessibility styling here.
