---
id: py/unaliased-collections-abc-set-import
title: "`collections.abc.Set` is imported as `AbstractSet`"
languages: [python]
fix: suggest
fix_reason: "the name changes in every use in the file, and another file may import it from this one"
stability: preview
ported_from: [ruff/PYI025]
since: 0.5.0
---

`from collections.abc import Set` is written `Set as AbstractSet`, and every use of the name in the file follows. The alias is not applied when the file already has a name `AbstractSet`.

**Why:** `Set` next to the builtin `set` is easy to read wrong; the alias keeps the abstract class apart.

```codelaw
native: py/unaliased-collections-abc-set-import
```

## Examples

### Python

#### Don't

```python
from collections.abc import Set


def size(items: Set[int]) -> int:
    return len(items)
```

#### Do

```python
from collections.abc import Set as AbstractSet


def size(items: AbstractSet[int]) -> int:
    return len(items)
```
