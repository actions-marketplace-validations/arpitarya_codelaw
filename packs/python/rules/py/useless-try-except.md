---
id: py/useless-try-except
title: "No handler that only re-raises"
languages: [python]
fix: suggest
fix_reason: "the `try` goes and its body moves up one level; a `try` with an `else` or `finally`, or another handler that does something, keeps its structure and is rewritten by hand"
stability: preview
ported_from: [ruff/TRY203, pylint/try-except-raise]
since: 0.5.0
---

An `except` handler whose body is only `raise` (or `raise e`, its own exception) is reported when every handler of the `try` does just that.

**Why:** Catching an exception to raise it again unchanged does nothing but add a level of nesting.

```codelaw
utils:
  reraise:
    kind: except_clause
    has: {kind: block, all: [{has: {nthChild: 1, kind: raise_statement, regex: '^raise(\s+\w+)?\s*$'}}, {not: {has: {nthChild: 2}}}]}
match:
  matches: reraise
  inside:
    kind: try_statement
    not: {has: {kind: except_clause, not: {matches: reraise}}}
  any:
    - inside:
        kind: try_statement
        pattern: $TRY
        all:
          - has: { field: body, pattern: $B }
          - not: { has: { kind: else_clause } }
          - not: { has: { kind: finally_clause } }
    - regex: ''
message: "The handler only re-raises; remove it"
fix:
  - unwrap-block: { target: $B, within: $TRY }
```

## How to fix

1. Remove the handler; if the `try` has no other handler, `else` or `finally`, remove the `try` too and dedent its body.

## Examples

### Python

#### Don't

```python
try:
    run()
except ValueError:
    raise
```

#### Do

```python
run()
```
