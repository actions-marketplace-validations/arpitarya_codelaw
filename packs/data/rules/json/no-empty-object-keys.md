---
id: json/no-empty-object-keys
title: "No empty key in a JSON object"
languages: [json]
fix: manual
fix_reason: "what the value belongs under is the author's knowledge"
fix_exception: value
stability: preview
ported_from: [biome/suspicious/noEmptyObjectKeys, eslint-plugin-json/no-empty-keys]
since: 0.5.1
---

An object does not have a key that is the empty string (`"": …`). Behaviour of Biome's `noEmptyObjectKeys`.

**Why:** An empty key is almost always a missing name: some parsers and mappers drop it or turn it into `undefined`.

```codelaw
match:
  kind: pair
  has: { field: key, kind: string, regex: '^""$' }
message: "This object has an empty key"
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JSON

#### Don't

```json
{
  "": 1
}
```

#### Do

```json
{
  "id": 1
}
```
