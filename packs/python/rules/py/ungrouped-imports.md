---
id: py/ungrouped-imports
title: "Imports of one package sit together"
languages: [python]
fix: manual
fix_reason: "where the import belongs is the author's, and a move can change what runs first"
fix_exception: restructure
stability: preview
ported_from: [pylint/ungrouped-imports]
since: 0.5.0
---

Taking the standard-library imports first, then the others, then relative ones, each in source order, an import of package `p` comes after the imports of another package since the last `p` import, with `import p` and `from p import x` counted separately. The package is the first part of the module name. Imports under `if TYPE_CHECKING:` and under an `os.name` or `sys.platform` test are not reported, and imports in a function or class are not read. A module is standard-library when the pack's table of standard-library names lists it; the project's own packages are not told from third-party ones.

**Why:** Imports of one package split by another's hide that two lines import from the same place, and make the block harder to sort.

```codelaw
native: py/ungrouped-imports
```

## How to fix

1. Move the import next to the other imports of its package.

## Examples

### Python

#### Don't

```python
import logging
import os
import logging.handlers
```

#### Do

```python
import logging
import logging.handlers
import os
```
