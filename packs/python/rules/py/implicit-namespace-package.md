---
id: py/implicit-namespace-package
title: "A package folder has an __init__.py"
languages: [python]
fix: manual
fix_reason: "the fix creates a file, `__init__.py`, beside this one, and whether the folder is a package or a namespace package on purpose is the author's"
fix_exception: authored
stability: preview
ported_from: [ruff/INP001]
since: 0.5.0
---

A `.py` file in a folder that holds no `__init__.py` (or `__init__.pyi`) is reported: that folder is an implicit namespace package (PEP 420), which tools that look for `__init__.py` — test discovery, type checkers, packaging — miss. Files at the project root or directly in `src/`, files that start with a shebang, and PEP 723 scripts are not reported, nor `.pyi` stubs. Ruff's `namespace-packages` setting has no equivalent: turn the rule off for those folders in a `[scope]`.

**Why:** A missing `__init__.py` makes a folder importable in one tool and invisible in another; an empty one says it is a package everywhere.

```codelaw
native: py/implicit-namespace-package
```

## How to fix

1. Add an empty `__init__.py` to the folder.
2. If the folder is a namespace package on purpose, turn the rule off for it in a `[scope]`.

## Examples

### Python

#### Don't

```python path=tools/report.py
def build():
    pass
```

#### Do

```python path=tools/__init__.py

```
