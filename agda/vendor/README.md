# Agda vendor libraries

Clone before typechecking TriWeavon formal modules:

```powershell
cd F:\Users\Matthew Ruhnau\LogOS\agda
pwsh -File scripts/vendor.ps1
```

Or with Make (Git Bash / WSL):

```bash
make vendor
```

Paths are relative to `TriWeavon.agda-lib`:

- `vendor/agda-cubical` — [agda/cubical](https://github.com/agda/cubical)
- `vendor/agda-stdlib` — [agda/agda-stdlib](https://github.com/agda/agda-stdlib)

## Check and HTML

```powershell
pwsh -File scripts/check.ps1
pwsh -File scripts/html.ps1
```

Reference library (not vendored): [1lab](https://1lab.dev)