# Dropped by the host re-seed — .claude/rules/host-win32.md, verbatim

- 2026-09-30: **A BOM-less `.ps1` holding any non-ASCII character (an em dash in a string) fails to PARSE under Windows PowerShell 5 — `Unexpected token`, nothing runs — so keep scratch `.ps1` files ASCII.**
- 2026-09-30: **`mklink /J` through the Bash tool dies with `Invalid switch` (MSYS mangles `/J`) — make a junction with PowerShell `New-Item -ItemType Junction`, and remove it with a non-recursive `[System.IO.Directory]::Delete(path, $false)` BEFORE removing the directory that holds it, or the recursive delete follows the link into its target.**
