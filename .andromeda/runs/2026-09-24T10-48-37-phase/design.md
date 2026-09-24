# design extract

## No domain coverage
Nothing in this chunk renders or styles anything in a design-system surface. It touches CI workflow steps, `.gitignore` entries, a stdlib-only dev-tooling scanner under `scripts/` and a `.claude/settings.json` hook fix. design-system covers only the `desktop-webview` surface and the `conductor-cli` product surface (§Surface: cli). Neither surface is touched here, and no amendment-history entry concerns CI, secret scanning or hooks.
