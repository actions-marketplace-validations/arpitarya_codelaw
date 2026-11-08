---
id: json/no-duplicate-keys
title: "No key twice in one JSON object"
languages: [json]
fix: auto
fix_reason: "auto when the key is set twice to the same value, so the later one goes; otherwise the choices keep the last (the default: what JSON.parse and Python read) or the first"
fix_exception: value
stability: preview
since: 0.1.0
---

Each key appears once in a JSON object. Keys are compared by value, so `"a"` and `"\u0061"` are the same key.

**Why:** RFC 8259 leaves duplicate keys undefined: most parsers keep the last value and silently drop the first, others keep the first or refuse the file.

```codelaw
native: json/no-duplicate-keys
```

## How to fix

1. Decide which value the key should have.
2. Delete the other entry, or move it under a key of its own.

## Examples

### JSON

#### Don't

```json
{
  "name": "app",
  "version": "1.0.0",
  "name": "web-app"
}
```

#### Fixed

```json
{
  "version": "1.0.0",
  "name": "web-app"
}
```

#### Do

```json
{
  "name": "web-app",
  "version": "1.0.0"
}
```
