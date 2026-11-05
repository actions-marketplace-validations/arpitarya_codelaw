---
id: py/multiple-with-statements
title: "One with statement, not nested ones"
languages: [python]
fix: manual
fix_reason: "merging re-indents the inner body, which a template edit cannot do"
fix_exception: restructure
stability: preview
ported_from: [ruff/SIM117]
since: 0.5.0
---

A `with` whose whole body is another `with` is written as one `with a, b:`.

**Why:** One statement says the resources are held together, with one level less of indentation.

```codelaw
match:
  kind: with_statement
  has:
    field: body
    not: {has: {nthChild: 2}}
    has: {kind: with_statement, nthChild: 1}
message: "Merge the nested `with` statements into one"
```

## How to fix

1. Write both context managers in one `with`, separated by a comma, and dedent the inner body one level.

## Examples

### Python

#### Don't

```python
with open(a) as src:
    with open(b, "w") as dst:
        dst.write(src.read())
```

#### Do

```python
with open(a) as src, open(b, "w") as dst:
    dst.write(src.read())
```
