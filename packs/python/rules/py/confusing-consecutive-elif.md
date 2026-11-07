---
id: py/confusing-consecutive-elif
title: "An `elif` does not follow a nested `if` that has no `else`"
languages: [python]
fix: manual
fix_reason: "which `if` the `elif` belongs to is the author's; the fix is an `else` on the inner one or a restructured chain"
fix_exception: restructure
stability: preview
ported_from: [pylint/confusing-consecutive-elif]
since: 0.5.0
---

The block of an `if` (or of an earlier `elif`) ends with a nested `if` chain that has no `else`, and the outer statement continues with `elif`. The `elif` sits one indent away from the inner `if`, and reads as if it continued it. This is one of Pylint's optional extensions; it is in no preset.

**Why:** Indentation is the only thing telling the reader which `if` the `elif` belongs to.

```codelaw
match:
  kind: elif_clause
  follows:
    stopBy: {any: [{kind: block}, {kind: elif_clause}]}
    any:
      - kind: block
        has:
          nthChild: {position: 1, reverse: true}
          kind: if_statement
          not: {has: {kind: else_clause}}
      - kind: elif_clause
        has:
          field: consequence
          has:
            nthChild: {position: 1, reverse: true}
            kind: if_statement
            not: {has: {kind: else_clause}}
message: "This `elif` follows a nested `if` without `else`; make the nesting explicit"
```

## How to fix

1. Give the inner `if` an `else`, or move it out of the block, so the `elif` clearly belongs to the outer `if`.

## Examples

### Python

#### Don't

```python
if ready:
    if fast:
        go()
elif waiting:
    wait()
```

#### Do

```python
if ready:
    if fast:
        go()
    else:
        pass
elif waiting:
    wait()
```
