---
id: py/uncapitalized-environment-variables
title: "Environment variable names are in capitals"
languages: [python]
fix: suggest
fix_reason: "on Linux and macOS environment names are case-sensitive, so the capitalised name may read another variable"
stability: preview
ported_from: [ruff/SIM112]
since: 0.5.0
---

The name read from `os.environ[...]`, `os.environ.get(...)` or `os.getenv(...)` is written in capitals.

**Why:** Environment variables are named in capitals by convention; a lowercase name is usually a typo for one.

```codelaw
match:
  kind: string_content
  pattern: $C
  regex: '[a-z]'
  inside:
    kind: string
    not: {has: {kind: interpolation}}
    any:
      - inside: {kind: subscript, field: subscript, has: {field: value, regex: '^os\.environ$'}}
      - all: [{nthChild: 1}, {inside: {kind: argument_list, inside: {kind: call, has: {field: function, regex: '^(os\.environ\.get|os\.getenv)$'}}}}]
message: "Write the environment variable name in capitals"
fix: "$C|upper"
```

## Examples

### Python

#### Don't

```python
key = os.environ["api_key"]
```

#### Do

```python
key = os.environ["API_KEY"]
```
