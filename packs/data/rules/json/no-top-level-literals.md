---
id: json/no-top-level-literals
title: "A JSON file is an object or an array at the top"
languages: [json]
fix: manual
fix_reason: "the value belongs under a key the author picks, or in an array"
fix_exception: value
stability: preview
ported_from: [biome/style/noTopLevelLiterals, eslint-plugin-json/top-level-interop]
since: 0.5.1
---

The top-level value of a JSON file is an object or an array, not a string, a number, `true`, `false` or `null`. Behaviour of Biome's `noTopLevelLiterals`.

**Why:** RFC 4627 allowed only an object or an array at the top; a parser written for it, and many tools that read config files, reject anything else.

```codelaw
target: $
assert: { type: [object, array] }
message: "The top-level value should be an object or an array, not `$VALUE`"
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JSON

#### Don't

```json
"production"
```

#### Do

```json
{ "mode": "production" }
```
