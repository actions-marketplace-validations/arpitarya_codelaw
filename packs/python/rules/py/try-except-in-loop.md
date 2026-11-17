---
id: py/try-except-in-loop
title: "No `try`/`except` at the top of a loop body"
languages: [python]
fix: manual
fix_reason: "moving the `try` outside the loop changes what one failure does to the iterations after it, which the author decides"
fix_exception: restructure
finding_target: { python: { below: "3.11" } }
stability: preview
ported_from: [ruff/PERF203]
since: 0.5.0
---

A `for` or `while` loop whose body starts with a `try` that has an `except` handler is reported. A `try` whose body holds a `break` or `continue` of that loop is left alone, since the handler is the loop's control flow there. Reported only for a target below Python 3.11, where entering a `try` costs an instruction on every iteration; from 3.11 the cost is zero. With no target the rule does nothing.

**Why:** On Python 3.10 and earlier, setting up the handler is paid on every pass through the loop.

```codelaw
match:
  kind: try_statement
  nthChild: 1
  has: { kind: except_clause }
  inside:
    kind: block
    inside:
      any: [{ kind: for_statement }, { kind: while_statement }]
      field: body
  not:
    has:
      field: body
      has:
        any: [{ kind: break_statement }, { kind: continue_statement }]
        stopBy: { any: [{ kind: for_statement }, { kind: while_statement }, { kind: function_definition }, { kind: class_definition }] }
message: "A `try` at the top of a loop body costs a setup on every iteration before Python 3.11; move it outside the loop"
```

## How to fix

1. Move the `try` outside the loop, or validate the value before the loop body instead of catching the failure.

## Examples

### Python

#### Don't

```python python=3.10
for raw in values:
    try:
        total += int(raw)
    except ValueError:
        pass
```

#### Do

```python python=3.10
def parse(raw):
    try:
        return int(raw)
    except ValueError:
        return 0


for raw in values:
    total += parse(raw)
```
