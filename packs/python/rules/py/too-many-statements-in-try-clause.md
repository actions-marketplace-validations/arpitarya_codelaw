---
id: py/too-many-statements-in-try-clause
title: "A try clause holds at most `max` statements"
languages: [python]
fix: manual
fix_reason: "splitting the code is a design choice: which parts belong together is the author's"
fix_exception: restructure
stability: preview
options:
  max: { type: "integer", default: 5, description: "the most statements a `try` clause may hold" }
ported_from: [ruff/PLW0717, pylint/too-many-try-statements]
since: 0.5.0
---

A `try` with at least one `except` whose body holds more than `max` statements (5 by default; Pylint's `max-try-statements` is 1), counted as `py/too-many-statements` counts them, is reported on its `try`.

**Why:** An `except` should catch the error of the one statement that can raise it; a long `try` catches it from lines that were not meant.

```codelaw
native: py/too-many-statements-in-try-clause
```

## How to fix

1. Keep in the `try` only the statement that raises what the `except` handles; move the rest before or after it, or into `else`.

## Examples

### Python

#### Don't

```python options={"max":1}
try:
    data = load()
    rows = parse(data)
except ValueError:
    rows = []
```

#### Do

```python options={"max":1}
data = load()
try:
    rows = parse(data)
except ValueError:
    rows = []
```
