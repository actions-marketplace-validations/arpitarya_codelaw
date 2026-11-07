---
id: py/loop-variable-overrides-iterator
title: "A loop variable doesn't replace what it iterates"
languages: [python]
fix: auto
fix_reason: "auto in a function that reads the name nowhere after the loop, from no nested scope outside it, with no locals(), vars(), eval(), exec(), global or nonlocal: the loop variable becomes the singular of the name (`items` → `item`), else `<name>_item`, in the loop. A module-level loop, or a read after the loop, is a suggestion, as the name then holds the iterable again; none when the new name is taken"
fix_exception: value
stability: preview
ported_from: [ruff/B020]
since: 0.1.0
---

A `for` loop's target doesn't reuse a name read in the iterable expression.

**Why:** After the loop the name holds the last item, not the collection, which breaks later uses.

```codelaw
native: py/loop-variable-overrides-iterator
```

## How to fix

1. Give the loop variable its own name.
2. Update its uses in the loop body.

## Examples

### Python

#### Don't

```py
for items in items:
    print(items)
```

#### Fixed

```py
for item in items:
    print(item)
```

#### Do

```py
for item in items:
    print(item)
```
