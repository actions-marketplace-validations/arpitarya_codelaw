---
id: json/use-required-scripts
title: "package.json has the scripts the project requires"
languages: [json]
files: ["**/package.json"]
fix: manual
fix_reason: "what the script runs is the project's"
fix_exception: authored
stability: preview
options:
  scripts: { type: "string[]", default: [], description: "script names every package.json must define, such as `build` and `test`" }
ported_from: [biome/suspicious/useRequiredScripts]
since: 0.5.1
---

Every script named in the `scripts` option is a key of `scripts` in `package.json`. With no option set, the rule does nothing. Behaviour of Biome's `useRequiredScripts`.

**Why:** A project that tells contributors to run `npm test` and `npm run build` needs every package to answer to them, and CI to be able to call them without checking.

```codelaw
native: json/use-required-scripts
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JSON

#### Don't

```json path=package.json options={"scripts":["build","test"]}
{
  "name": "app",
  "scripts": { "build": "tsc" }
}
```

#### Do

```json path=package.json options={"scripts":["build","test"]}
{
  "name": "app",
  "scripts": { "build": "tsc", "test": "vitest" }
}
```
