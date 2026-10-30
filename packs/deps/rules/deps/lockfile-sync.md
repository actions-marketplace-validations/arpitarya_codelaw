---
id: deps/lockfile-sync
title: "The lockfile matches its manifest"
languages: [json, toml]
files: ["**/package.json", "**/pyproject.toml"]
fix: manual
fix_reason: "only the package manager can write its lockfile, and codelaw never runs installers"
stability: preview
since: 0.5.0
---

A manifest edited without running the install leaves a lockfile that installs something else: CI's `npm ci` refuses it, or a teammate gets a different tree. The rule compares each `package.json` and `pyproject.toml` with the lockfile that installs it — beside it, or a workspace's at the root — through what the lockfile records of the manifest it was made from:

| lockfile | what it records | reported |
|---|---|---|
| `package-lock.json` | `packages[""]` (a workspace's `packages["<dir>"]`): each dependency's range as written | missing, another range, a name the manifest dropped |
| `pnpm-lock.yaml` | `importers` (v5: `specifiers`): each `specifier` | missing, another range, a name the manifest dropped |
| `yarn.lock` | an entry per `name@range` (berry: `name@npm:range`) | a dependency with no entry |
| `uv.lock` | the project's `[package.metadata] requires-dist` and `requires-dev` | missing, another range, a name the manifest dropped |
| `poetry.lock` | `[metadata] content-hash`, a SHA-256 of the dependency tables | a hash that does not match, recomputed as Poetry 1.x and 2.x do |

A dependency's finding is on its manifest line; one about the whole file on its first line. The message names the command that brings the lockfile up to date: `npm install`, `pnpm install`, `yarn install`, `uv lock` or `poetry lock`. The rule reads only files and never edits a lockfile.

**Why:** The lockfile is what installs. A manifest it does not match says one thing and ships another.

```codelaw
native: deps/lockfile-sync
```

## How to fix

1. Run the command the message names in the folder of the lockfile.
2. Commit the manifest and the lockfile together.

## Examples

### JSON

#### Don't

```json
// package-lock.json
{
  "lockfileVersion": 3,
  "packages": {
    "": { "dependencies": { "react": "^18.2.0" } }
  }
}
// package.json
{
  "dependencies": {
    "react": "^18.3.1"
  }
}
```

#### Do

```json
// package-lock.json
{
  "lockfileVersion": 3,
  "packages": {
    "": { "dependencies": { "react": "^18.3.1" } }
  }
}
// package.json
{
  "dependencies": {
    "react": "^18.3.1"
  }
}
```

### TOML

#### Don't

```toml
# uv.lock
version = 1

[[package]]
name = "app"
version = "0.1.0"
source = { virtual = "." }

[package.metadata]
requires-dist = [{ name = "requests", specifier = ">=2.31" }]
# pyproject.toml
[project]
name = "app"
dependencies = ["requests>=2.32"]
```

#### Do

```toml
# uv.lock
version = 1

[[package]]
name = "app"
version = "0.1.0"
source = { virtual = "." }

[package.metadata]
requires-dist = [{ name = "requests", specifier = ">=2.32" }]
# pyproject.toml
[project]
name = "app"
dependencies = ["requests>=2.32"]
```
