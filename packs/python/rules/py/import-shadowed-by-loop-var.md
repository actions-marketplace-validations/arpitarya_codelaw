---
id: py/import-shadowed-by-loop-var
title: "A loop variable doesn't reuse an imported name"
languages: [python]
fix: auto
fix_reason: "auto in a function that reads the name nowhere after the loop, from no nested scope outside it, with no locals(), vars(), eval(), exec(), global or nonlocal: the loop variable becomes `<name>_` in the loop, and the import holds everywhere else as before; a module-level loop rebinds a global others may read, and a read after the loop would see the module, so those are suggestions. None when `<name>_` is taken"
fix_exception: value
stability: preview
ported_from: [ruff/F402]
since: 0.1.0
---

A `for` loop's target doesn't rebind a name that an import in the same scope bound.

**Why:** After the loop the name no longer refers to the module, so later uses break in ways that are hard to trace.

```codelaw
native: py/import-shadowed-by-loop-var
```

## How to fix

1. Rename the loop variable.
2. Update its uses inside the loop body.

## Examples

### Python

#### Don't

```py
import path

for path in ["a.txt", "b.txt"]:
    print(path)
```

#### Fixed

```py
import path

for path_ in ["a.txt", "b.txt"]:
    print(path_)
```

#### Do

```py
import path

for name in ["a.txt", "b.txt"]:
    print(path.join(name))
```
