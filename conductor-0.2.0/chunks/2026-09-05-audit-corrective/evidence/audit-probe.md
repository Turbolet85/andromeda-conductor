# The standing cargo-audit probe — transcript

Run at 2026-09-05T20:32:58Z, the form `plan.md` §Test Commands block 7 lists.

```
DB="${CARGO_HOME:-$HOME/.cargo}/advisory-db"
git -C "$DB" rev-parse HEAD    -> 5a0ebedfe8bdd2e295b171f4162f8c977bcad9a5
git -C "$DB" status --porcelain -> (no output: 0 lines)

cargo audit                    -> exit 0
      Loaded 1239 security advisories (from <cargo-home>/advisory-db)
    Scanning Cargo.lock for vulnerabilities (564 crate dependencies)
warning: 18 allowed warnings found

cargo deny check advisories bans licenses sources -> exit 0
advisories ok, bans ok, licenses ok, sources ok
```

`Cargo.lock`: 564 packages, untouched. No crate manifest changed.
