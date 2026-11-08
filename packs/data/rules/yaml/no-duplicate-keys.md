---
id: yaml/no-duplicate-keys
title: "No key twice in one YAML mapping"
languages: [yaml]
fix: auto
fix_reason: "auto when the key is set twice to the same value, so the later one goes; otherwise the choices keep the last (the default: what most YAML readers keep) or the first"
fix_exception: value
stability: preview
since: 0.1.0
---

Each key appears once in a mapping, block or flow. Quoted and plain spellings of a key are the same key.

**Why:** YAML 1.2 requires keys to be unique, but many loaders accept duplicates and keep only the last value, so the first one is silently lost.

```codelaw
native: yaml/no-duplicate-keys
```

## How to fix

1. Decide which value the key should have.
2. Delete the other entry, or give it a key of its own.

## Examples

### YAML

#### Don't

```yaml
service:
  image: nginx
  restart: always
  image: caddy
```

#### Fixed

```yaml
service:
  restart: always
  image: caddy
```

#### Do

```yaml
service:
  image: caddy
  restart: always
```
