---
id: deps/runtime-agreement
title: "Every place that names the Node or Python version agrees"
languages: [json, toml, yaml]
files: ["package.json", "pyproject.toml", "mise.toml", ".mise.toml", ".github/workflows/*.yml", ".github/workflows/*.yaml"]
fix: suggest
fix_reason: "changing a runtime pin changes what CI, the container or a developer's shell runs; the agreed version may not be the one the team meant"
stability: preview
since: 0.5.0
---

A project names its runtime in many places, and they drift: CI tests Node 18, the container runs 20, `engines` says 22. The rule reads, at the project root:

| | Node | Python |
|---|---|---|
| the range | `engines.node` in `package.json` | `requires-python` in `pyproject.toml` |
| pins | `volta.node`, `.nvmrc`, `.node-version`, `.tool-versions` (`nodejs`), `mise.toml` `[tools] node`, `actions/setup-node`'s `node-version` in `.github/workflows/*.yml`, `Dockerfile` `FROM node:<tag>` | `.python-version`, `.tool-versions` (`python`), `mise.toml` `[tools] python`, `actions/setup-python`'s `python-version`, `FROM python:<tag>` |

Every pin must be **inside the range** and **agree with the others**: `20` agrees with `20.11.1`; `20.10` does not. An alias (`lts/*`, `latest`) or an expression (`${{ matrix.node }}`) is not read. The pin the others are compared with is the first, in the order above, that the range allows — or the source `[deps.runtime] source` names:

```toml
[deps.runtime]
source = { node = ".nvmrc", python = "requires-python" }   # the truth; defaults engines.node, requires-python
```

Where it is reported, and the fix:

- **A pin in `package.json`, `mise.toml` or a workflow:** on the pin, and the fix sets it to the agreed version at the precision it was written.
- **A pin in `.nvmrc`, `.node-version`, `.python-version`, `.tool-versions` or the `Dockerfile`:** on the root `package.json` (Node) or `pyproject.toml` (Python) — on the range when it names one — and on the pin's own line; change it by hand: codelaw never edits those files.
- **A range that does not allow the pin `[deps.runtime] source` names:** on the range, to widen by hand.

**Why:** The version CI tests is the only one known to work; every other place should name the same one.

```codelaw
native: deps/runtime-agreement
```

## How to fix

1. Run `codelaw fix --unsafe` to move the pins in `package.json`, `mise.toml` and the workflows to the agreed version.
2. Edit the version files and the `Dockerfile`'s `FROM` line to the same version by hand.
3. If the range is what is wrong, widen `engines.node` or `requires-python`.

## Examples

### JSON

#### Don't

```json
// .nvmrc
20.11.1
// package.json
{
  "engines": { "node": ">=20.11" },
  "volta": { "node": "18.19.0" }
}
```

#### Do

```json
// .nvmrc
20.11.1
// package.json
{
  "engines": { "node": ">=20.11" },
  "volta": { "node": "20.11.1" }
}
```

### YAML

#### Don't

```yaml
# .nvmrc
20
# .github/workflows/ci.yml
on: push
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/setup-node@v4
        with:
          node-version: 18
```

#### Do

```yaml
# .nvmrc
20
# .github/workflows/ci.yml
on: push
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/setup-node@v4
        with:
          node-version: 20
```

### TOML

#### Don't

```toml
# .python-version
3.12
# pyproject.toml
[project]
name = "app"
requires-python = ">=3.11"
# mise.toml
[tools]
python = "3.10"
```

#### Do

```toml
# .python-version
3.12
# pyproject.toml
[project]
name = "app"
requires-python = ">=3.11"
# mise.toml
[tools]
python = "3.12"
```
