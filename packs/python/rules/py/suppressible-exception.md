---
id: py/suppressible-exception
title: "contextlib.suppress, not try/except/pass"
languages: [python]
fix: manual
fix_reason: "the fix needs `contextlib` imported and the body re-indented under `with`"
fix_exception: restructure
stability: preview
ported_from: [ruff/SIM105]
since: 0.5.0
---

A `try` with one statement and one `except E: pass` is written `with contextlib.suppress(E):`.

**Why:** `suppress` says the exception is expected and ignored, in one line.

```codelaw
match:
  kind: try_statement
  all:
    - has: {field: body, kind: block, not: {has: {nthChild: 2}}}
    - has: {kind: except_clause, regex: '^except\s+[\w.(), ]+:\s*pass$'}
    - not: {has: {nthChild: 3}}
    - not: {has: {any: [{kind: else_clause}, {kind: finally_clause}]}}
message: "Use `with contextlib.suppress(...)`"
```

## How to fix

1. Import `contextlib`.
2. Replace the `try`/`except E: pass` with `with contextlib.suppress(E):` around the statement.

## Examples

### Python

#### Don't

```python
try:
    os.remove(path)
except FileNotFoundError:
    pass
```

#### Do

```python
with contextlib.suppress(FileNotFoundError):
    os.remove(path)
```
