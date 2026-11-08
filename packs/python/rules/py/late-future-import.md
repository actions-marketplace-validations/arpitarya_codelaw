---
id: py/late-future-import
title: "`from __future__` imports come first"
languages: [python]
fix: auto
fix_reason: "a late `from __future__` import is a SyntaxError, so the module never ran: it moves above the first statement that isn't a docstring or another `__future__` import, its only meaning. One inside a block, or sharing its line, is manual"
fix_exception: restructure
stability: preview
ported_from: [ruff/F404, pylint/misplaced-future]
since: 0.1.0
---

`from __future__ import …` statements come before any other statement except the module docstring.

**Why:** Python rejects a late `__future__` import with a `SyntaxError`.

```codelaw
native: py/late-future-import
```

## How to fix

1. Move the `from __future__` import to the top, after the docstring.

## Examples

### Python

#### Don't

```py
import os
from __future__ import annotations
```

#### Do

```py
from __future__ import annotations
import os
```
