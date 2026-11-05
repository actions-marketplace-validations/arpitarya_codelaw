---
id: py/unused-lambda-argument
title: "No unused lambda parameter"
languages: [python]
fix: manual
fix_reason: "the lambda's callers decide its signature"
fix_exception: restructure
stability: preview
ported_from: [ruff/ARG005]
since: 0.5.0
---

A lambda's parameter is used in its body; `_`-prefixed names are left alone.

**Why:** An unused parameter usually means the wrong name was used in the body.

```codelaw
match:
  kind: identifier
  regex: '^[a-zA-Z]\w*$'
  pattern: $P
  any:
    - inside: {kind: lambda_parameters}
    - inside: {any: [{kind: default_parameter}], inside: {kind: lambda_parameters}}
  inside: {kind: lambda, stopBy: end, has: {field: body, all: [{not: {has: {stopBy: end, kind: identifier, pattern: $P}}}, {not: {pattern: $P}}]}}
message: "`$P` is never used"
```

## How to fix

1. Use the parameter, rename it with a leading `_`, or remove it if the caller allows.

## Examples

### Python

#### Don't

```python
handler = lambda event, ctx: print(ctx)
```

#### Do

```python
handler = lambda _event, ctx: print(ctx)
```
