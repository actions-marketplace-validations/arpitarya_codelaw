---
id: py/missing-required-import
title: "A file has the imports the project requires"
languages: [python]
fix: auto
stability: preview
options: { required_imports: { type: "string[]", default: [], description: "import statements every file needs, such as `from __future__ import annotations`" } }
ported_from: [ruff/I002]
since: 0.5.0
---

Every Python file has each import statement listed in `required_imports` (an empty list by default, so the rule does nothing until the project sets it). The import is added below the module docstring, or at the top of the file below its leading comments. A file that holds nothing but a docstring is left alone, and a `.pyi` stub is not asked for `from __future__` imports.

**Why:** A project that wants every module to start with the same import (`from __future__ import annotations`) keeps that true as files are added.

```codelaw
native: py/missing-required-import
```

## Examples

### Python

#### Don't

```python options={"required_imports":["from\u0020__future__\u0020import\u0020annotations"]}
import os
```

#### Do

```python options={"required_imports":["from\u0020__future__\u0020import\u0020annotations"]}
from __future__ import annotations

import os
```
