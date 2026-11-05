---
id: deps/licence
title: "Every dependency has a licence the project allows"
languages: [json, toml]
files: ["**/package.json", "**/pyproject.toml"]
fix: manual
fix_reason: "replacing a package, or accepting its licence, is a legal and product decision no tool can make"
fix_exception: value
stability: preview
since: 0.5.0
---

The licence of every version a project installs — each dependency of a `package.json` or `pyproject.toml`, and every package its lockfile installs through them — is read from `.codelaw/deps-snapshot.json`, where `codelaw deps refresh` keeps the registry's licence for each version in use, and judged against `[deps.licence]`. The rule reads only files, so `check` gives the same answer offline.

A licence is an **SPDX expression**: `MIT`, `(MIT OR Apache-2.0)`, `Apache-2.0 AND BSD-3-Clause`, `GPL-2.0-only WITH Classpath-exception-2.0`. `OR` needs one allowed alternative; `AND` needs every part allowed; `WITH` keeps its exception with its id. Free text such as `BSD` or `see LICENSE` is never guessed at: it is **unknown** (a short table of exact texts that name one licence without doubt, such as PyPI's `MIT License` classifier, is read as that licence).

Each id is judged by the configured lists first, then the defaults:

| list | default |
|---|---|
| `deny` | `GPL-*`, `AGPL-*`, `LGPL-*`, `SSPL-*`, `BUSL-*`, `EUPL-*` (copyleft and source-available) |
| `allow` | `MIT`, `Apache-2.0`, `BSD-2-Clause`, `BSD-3-Clause`, `ISC`, `0BSD`, `Unlicense`, `CC0-1.0`, `PSF-2.0`, `Python-2.0`, `BlueOak-1.0.0` |

```toml
[deps.licence]
allow = ["MPL-2.0", "LGPL-*"]        # globs over SPDX ids; lifts a default denial
deny = ["Unlicense"]
unknown = "warn"                     # error | warn | info | off
ignore = [{ package = "npm:some-gpl-tool", expires = "2027-01-01", reason = "dev-only, replaced in Q1" }]
```

A denied licence is reported at the rule's severity (`error`); an unknown one — on neither list, free text, or a direct dependency the registry names no licence for — at `[deps.licence] unknown`. A direct dependency's finding is on its manifest line. The transitive ones are one finding on the first line of the manifest beside the lockfile, naming up to ten packages and how many more, graded by the worst; for yarn classic each named package has a copy on its `yarn.lock` entry. `[deps] ignore` (whole packages) and `[deps.licence] ignore` hold a package.

**Why:** A copyleft licence deep in the tree can oblige the whole product; an unknown one is a question nobody has answered yet. Both are cheaper to answer when the package arrives than after it ships.

```codelaw
native: deps/licence
```

## How to fix

1. Read the package's licence. If the project can accept it, add its id to `[deps.licence] allow`.
2. Otherwise replace the package (or the dependency that brings it in), then run the install.
3. For a known exception, add a dated `[deps.licence] ignore` entry with `reason` and `added_by`.

## Examples

### JSON

#### Don't

```json
// .codelaw/deps-snapshot.json
{
  "fetched": "2026-10-29",
  "packages": {
    "npm:gpl-lib": { "versions": { "1.0.0": "2024-01-01" }, "licence": { "1.0.0": "GPL-3.0-only" } }
  },
  "sources": { "osv": { "kind": "osv", "state": "ok", "on": "2026-10-29" } }
}
// package.json
{
  "dependencies": {
    "gpl-lib": "^1.0.0"
  }
}
```

#### Do

```json
// .codelaw/deps-snapshot.json
{
  "fetched": "2026-10-29",
  "packages": {
    "npm:mit-lib": { "versions": { "1.0.0": "2024-01-01" }, "licence": { "1.0.0": "MIT OR GPL-3.0-only" } }
  },
  "sources": { "osv": { "kind": "osv", "state": "ok", "on": "2026-10-29" } }
}
// package.json
{
  "dependencies": {
    "mit-lib": "^1.0.0"
  }
}
```

### TOML

#### Don't

```toml
# .codelaw/deps-snapshot.json
{
  "fetched": "2026-10-29",
  "packages": {
    "pypi:odd": { "versions": { "2.0": "2024-01-01" }, "licence": { "2.0": "see LICENSE.txt" } }
  },
  "sources": { "osv": { "kind": "osv", "state": "ok", "on": "2026-10-29" } }
}
# pyproject.toml
[project]
name = "app"
dependencies = ["odd>=2.0"]
```

#### Do

```toml
# .codelaw/deps-snapshot.json
{
  "fetched": "2026-10-29",
  "packages": {
    "pypi:fine": { "versions": { "2.0": "2024-01-01" }, "licence": { "2.0": "Apache-2.0" } }
  },
  "sources": { "osv": { "kind": "osv", "state": "ok", "on": "2026-10-29" } }
}
# pyproject.toml
[project]
name = "app"
dependencies = ["fine>=2.0"]
```
