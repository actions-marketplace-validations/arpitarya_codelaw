---
id: json/no-duplicate-dependencies
title: "A dependency is listed in one group only"
languages: [json]
files: ["**/package.json"]
fix: manual
fix_reason: "which group the package belongs in is the author's decision"
fix_exception: value
stability: preview
ported_from: [biome/suspicious/noDuplicateDependencies, eslint-plugin-package-json/unique-dependencies]
since: 0.5.1
---

A package name appears in only one of `dependencies`, `devDependencies`, `peerDependencies` and `optionalDependencies`; the second listing is reported. A name in both `peerDependencies` and `devDependencies` is the usual way to test a plugin against its host and is allowed. Behaviour of Biome's `noDuplicateDependencies`.

**Why:** Two entries for one package can ask for two versions, and which one the package manager installs is not what the file's reader would guess.

```codelaw
native: json/no-duplicate-dependencies
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JSON

#### Don't

```json path=package.json
{
  "dependencies": { "lodash": "^4.17.21" },
  "devDependencies": { "lodash": "^4.17.21" }
}
```

#### Do

```json path=package.json
{
  "dependencies": { "lodash": "^4.17.21" }
}
```
