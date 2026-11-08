---
id: py/banned-import-from
title: "No from-import of the modules the project lists"
languages: [python]
fix: manual
fix_reason: "each use of an imported name is rewritten through the module"
fix_exception: restructure
stability: preview
options: { modules: { type: "string[]", default: [] } }
ported_from: [ruff/ICN003]
since: 0.5.0
---

A module the project lists in `modules` is imported whole and its members are reached through it: `import os.path`, then `os.path.join`, not `from os.path import join`. The list is empty by default.

**Why:** Some modules read better qualified at every use, so a reader sees where a name comes from.

```codelaw
match:
  kind: import_from_statement
  has: {field: module_name, pattern: $M}
where:
  $M: {in: modules}
message: "Import `$M` and qualify its names"
```

## How to fix

1. Replace the `from` import with `import <module>` and write each use as `<module>.<name>`.

## Examples

### Python

#### Don't

```python options={"modules":["os.path"]}
from os.path import join
```

#### Do

```python options={"modules":["os.path"]}
import os.path
```
