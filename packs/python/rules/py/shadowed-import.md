---
id: py/shadowed-import
title: "An import does not take the name of an earlier import"
languages: [python]
fix: manual
fix_reason: "which of the two names to change is the author's"
fix_exception: value
stability: preview
ported_from: [pylint/shadowed-import]
since: 0.5.0
---

An import statement binds a name that an earlier import in the same block already bound, from a different module or object: `from pathlib import Path` followed by `from lib import Path`, or `from pandas._libs import algos` followed by `import pandas.core.algorithms as algos`. The same import twice is `py/reimported`. Imports in different blocks (the two arms of `try`/`except ImportError`, or of an `if`) are not compared.

**Why:** Every later use of the name means the second import and the first is dead; the code reads as if both were available.

```codelaw
native: py/shadowed-import
```

## How to fix

1. Give one of them an alias, or
2. drop the import that is not used.

## Examples

### Python

#### Don't

```python
from pathlib import Path
from custom import CustomPath as Path
```

#### Do

```python
from pathlib import Path
from custom import CustomPath
```
