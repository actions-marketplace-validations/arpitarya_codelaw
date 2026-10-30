---
id: deps/version-drift
title: "A monorepo declares each dependency with one range"
languages: [json, toml]
files: ["**/package.json", "**/pyproject.toml"]
fix: suggest
fix_reason: "aligning a range changes what that workspace installs; the lower range may be on purpose"
stability: preview
since: 0.5.0
---

In a monorepo — npm, yarn or pnpm workspaces (`package.json` `workspaces`, `pnpm-workspace.yaml` `packages`), or a uv workspace (`[tool.uv.workspace] members`) — the same dependency declared with different ranges in different workspaces installs two versions, or one version some workspace was never tested with. The rule compares each dependency's range across the root manifest and every member: `dependencies`, `devDependencies` and `optionalDependencies` (never `peerDependencies`, which are wide on purpose), and `[project] dependencies`, `optional-dependencies` and `[dependency-groups]` (PyPI ranges by their clauses, so `>= 2.31` and `>=2.31.0` agree). A range with a protocol (`workspace:`, `file:`, a URL) is not compared.

Each manifest whose range is not the target gets a finding, and the fix rewrites it to the target:

```toml
[deps.drift]
policy = "highest"          # the range whose floor is highest (default); "root": the root manifest's
ignore = ["npm:typescript"] # never compared
```

**Why:** One version per dependency is one set of bugs, one set of types and one upgrade.

```codelaw
native: deps/version-drift
```

## How to fix

1. Run `codelaw fix --unsafe` to align the range, or edit it by hand.
2. Run the install so the lockfile follows.
3. If a workspace needs another version on purpose, add the package to `[deps.drift] ignore`.

## Examples

### JSON

#### Don't

```json
// package.json
{
  "private": true,
  "workspaces": ["packages/*"]
}
// packages/web/package.json
{
  "dependencies": {
    "react": "^18.2.0"
  }
}
// packages/app/package.json
{
  "dependencies": {
    "react": "^18.3.1"
  }
}
```

#### Do

```json
// package.json
{
  "private": true,
  "workspaces": ["packages/*"]
}
// packages/web/package.json
{
  "dependencies": {
    "react": "^18.3.1"
  }
}
// packages/app/package.json
{
  "dependencies": {
    "react": "^18.3.1"
  }
}
```

### TOML

#### Don't

```toml
# pyproject.toml
[project]
name = "mono"
dependencies = ["httpx>=0.27"]

[tool.uv.workspace]
members = ["libs/*"]
# libs/core/pyproject.toml
[project]
name = "core"
dependencies = ["httpx>=0.25"]
```

#### Do

```toml
# pyproject.toml
[project]
name = "mono"
dependencies = ["httpx>=0.27"]

[tool.uv.workspace]
members = ["libs/*"]
# libs/core/pyproject.toml
[project]
name = "core"
dependencies = ["httpx>=0.27"]
```
