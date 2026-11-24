---
id: json/no-json-unsafe-values
title: "No value JSON parsers read differently"
languages: [json]
fix: manual
fix_reason: "what number or text was meant is the author's knowledge"
fix_exception: value
stability: preview
ported_from: [biome/nursery/noJsonUnsafeValues, eslint-plugin-json/no-unsafe-values]
since: 0.5.1
---

A number that is too large for a double (`1e999`, read as `Infinity`), too small (`1e-999`, read as `0`), `-0`, or an integer beyond ±9007199254740991, and a string with a lone `\uD800`–`\uDFFF` escape, are reported. Behaviour of Biome's `noJsonUnsafeValues`.

**Why:** JSON is an interchange format: a value that one parser keeps and another rounds, replaces or rejects is a bug waiting for the second parser.

```codelaw
native: json/no-json-unsafe-values
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JSON

#### Don't

```json
{ "id": 12345678901234567890 }
```

#### Do

```json
{ "id": "12345678901234567890" }
```
