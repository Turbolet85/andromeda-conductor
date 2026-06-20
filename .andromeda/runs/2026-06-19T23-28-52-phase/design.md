# design extract

## No domain coverage
The abrupt-silence fault helper is a backend domain primitive (Rust struct + constructor); design system covers only UI surfaces (desktop-webview + cli rendering), tokens, typography, and interaction patterns. Backend types require no design system guidance. (Distiller also emitted a one-line `## Relevance: out-of-scope` — normalized here to the canonical `No domain coverage` form.)