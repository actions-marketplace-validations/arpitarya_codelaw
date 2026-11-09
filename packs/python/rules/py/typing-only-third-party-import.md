---
id: py/typing-only-third-party-import
title: "An import of a third-party module used only in annotations is under `TYPE_CHECKING`"
languages: [python]
fix: suggest
fix_reason: "the import moves to where it is not run, which changes what `import` runs at start-up; the first finding of a file holds the edit for all of them"
stability: preview
ported_from: [ruff/TC002]
since: 0.5.0
---

An import of a module that is not the standard library's and not the project's own, whose every use is in a type annotation, is moved under `if TYPE_CHECKING:`. A use counts as typing-only in an annotation when the file has `from __future__ import annotations`, in an annotation inside a function body, in a quoted annotation, and under `if TYPE_CHECKING:`; any other use runs, and keeps the import. A name in `__all__`, an import from `typing`, and an import of a module whose other imports are used at run time are not moved (Ruff's default, non-strict mode). The first finding of the file carries the edit that moves every typing-only import of the file, below the file's `if TYPE_CHECKING:` block or a new one after its last remaining import, importing `TYPE_CHECKING` from `typing`; the others report only.

**Why:** An import used only to describe types is paid for at start-up and can make an import cycle; under `TYPE_CHECKING` only the type checker reads it.

```codelaw
native: py/typing-only-third-party-import
```

## Examples

### Python

#### Don't

```python
from __future__ import annotations

from requests import Session


def close(session: Session) -> None:
    session.close()
```

#### Do

```python
from __future__ import annotations

from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from requests import Session


def close(session: Session) -> None:
    session.close()
```
