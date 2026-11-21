---
id: json/use-package-json-valid-engines
title: "`engines` in package.json holds version ranges"
languages: [json]
files: ["**/package.json"]
fix: manual
fix_reason: "which range is meant is the author's knowledge"
fix_exception: value
stability: preview
ported_from: [biome/nursery/usePackageJsonValidEngines, eslint-plugin-package-json/valid-engines]
since: 0.5.1
---

`engines` is an object, and each value is a string that has the shape of a semver range: digits or `x`/`*` with `<`, `>`, `=`, `~`, `^`, `||`, `-`, spaces and pre-release tags. `"latest"`, `"lts/*"` and a number are reported. The shape is checked, not whether the range selects anything. Behaviour of Biome's `usePackageJsonValidEngines`.

**Why:** npm and Node's package managers read `engines` as ranges; a value they cannot parse is ignored, so the guard the author wrote does nothing.

```codelaw
target: engines
assert:
  type: object
  path:
    "*":
      type: string
      matches: '^\s*[<>=~^v|\s*xX0-9.+A-Za-z-]*[0-9xX*][<>=~^v|\s*xX0-9.+A-Za-z-]*$'
message: "`engines` needs an object of version ranges; `$VALUE` is not one"
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JSON

#### Don't

```json path=package.json
{
  "name": "app",
  "engines": { "node": "latest" }
}
```

#### Do

```json path=package.json
{
  "name": "app",
  "engines": { "node": ">=22" }
}
```
