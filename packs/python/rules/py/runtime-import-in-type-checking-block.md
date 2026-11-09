---
id: py/runtime-import-in-type-checking-block
title: "A name used at run time is not imported only under `TYPE_CHECKING`"
languages: [python]
fix: suggest
fix_reason: "the import moves out of the block to where it runs; the first finding of a file holds the edit for all of them"
stability: preview
ported_from: [ruff/TC004]
since: 0.5.0
---

A name imported inside a module-level `if TYPE_CHECKING:` block is used somewhere that runs (a call, a base class, a default, an annotation that runs). The edit moves the import below the file's last import, and a block left empty keeps `pass`. A use in an annotation counts as running only when the file has no `from __future__ import annotations`.

**Why:** At run time the name does not exist, and the code fails with a `NameError`.

```codelaw
native: py/runtime-import-in-type-checking-block
```

## Examples

### Python

#### Don't

```python
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from pathlib import Path

home = Path("/")
```

#### Do

```python
from typing import TYPE_CHECKING
from pathlib import Path

if TYPE_CHECKING:
    pass

home = Path("/")
```
