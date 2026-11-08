---
id: py/confusing-with-statement
title: "A `with` item after an `as` name is not mistaken for a tuple target"
languages: [python]
fix: manual
fix_reason: "whether `b` was meant as a second context manager or as part of the target is the author's"
fix_exception: value
stability: preview
ported_from: [pylint/confusing-with-statement]
since: 0.5.0
---

In `with a as x, b:` the item after `as x` is a second context manager, though it reads like `with a as (x, b):`. Reported when an item binds a plain name and the next item has no `as` and is not a call.

**Why:** A reader, or the author, can take `x, b` for one unpacked target; `b` is in fact entered as a context manager.

```codelaw
match:
  kind: with_item
  has:
    field: value
    kind: as_pattern
    has: {field: alias, has: {kind: identifier}}
  precedes:
    stopBy: {kind: with_item}
    kind: with_item
    has:
      field: value
      not:
        any:
          - kind: as_pattern
          - kind: call
message: "The item after `as` is a second context manager, not part of the target"
```

## How to fix

1. If two context managers were meant, give the second its own `as` or nest the `with` statements.
2. If one target was meant, write it in parentheses: `with a as (x, b):`.

## Examples

### Python

#### Don't

```python
with lock as held, other:
    run(held)
```

#### Do

```python
with lock as held:
    with other:
        run(held)
```
