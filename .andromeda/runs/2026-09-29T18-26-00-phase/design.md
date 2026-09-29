# design extract

## No domain coverage
The chunk adds two root license files and a `license` metadata field to the Cargo and `package.json` manifests. It renders nothing on either surface (desktop-webview or cli), and it touches no tokens, typography, motion, iconography or component patterns. design-system.md has no license or manifest content (a grep for `licen`, `about` and `package.json` returns 0), and the scope excludes any installer license page.
