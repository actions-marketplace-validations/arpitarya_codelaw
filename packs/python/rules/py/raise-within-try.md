---
id: py/raise-within-try
title: "A `try` does not catch what its own body raises"
languages: [python]
fix: manual
fix_reason: "moving the check out of the `try`, or into a helper, is a restructure of the author's control flow"
fix_exception: restructure
stability: preview
ported_from: [ruff/TRY301]
since: 0.5.0
---

A `raise` in a `try` body raises a type that one of the `try`'s own handlers catches — the same name, or any exception when a handler catches `Exception` or `BaseException`. The exception goes straight to the handler: the `raise` is a `goto`. A bare `raise` is left alone, and so is a `raise` inside a nested `try`, which is read against that `try`'s handlers. Names are compared as written; a handler for a base class of the raised type is not recognised, as upstream.

**Why:** Raising to your own handler hides the control flow; a reader expects a `raise` to leave the block.

```codelaw
match:
  kind: raise_statement
  all:
    - not:
        inside:
          stopBy: {kind: try_statement}
          any: [{kind: except_clause}, {kind: else_clause}, {kind: finally_clause}]
    - has:
        any:
          - {kind: call, has: {field: function, pattern: $E}}
          - {any: [{kind: identifier}, {kind: attribute}], pattern: $E}
    - inside:
        stopBy: {kind: try_statement}
        kind: try_statement
        has:
          kind: except_clause
          has:
            any:
              - regex: '^(builtins\.)?(Exception|BaseException)$'
              - pattern: $E
              - {kind: as_pattern, has: {nthChild: 1, any: [{regex: '^(builtins\.)?(Exception|BaseException)$'}, {pattern: $E}, {kind: tuple, has: {any: [{regex: '^(builtins\.)?(Exception|BaseException)$'}, {pattern: $E}]}}]}}
              - {kind: tuple, has: {any: [{regex: '^(builtins\.)?(Exception|BaseException)$'}, {pattern: $E}]}}
message: "This `raise` is caught by its own `try`"
```

## How to fix

1. Move the check that raises out of the `try`, or into a function the `try` calls, so the handler only handles what the protected code raises.

## Examples

### Python

#### Don't

```python
try:
    value = parse(text)
    if value < 0:
        raise ValueError(value)
except ValueError:
    value = 0
```

#### Do

```python
try:
    value = parse(text)
except ValueError:
    value = 0
if value < 0:
    value = 0
```
