---
id: py/undocumented-public-module
title: "A public module has a docstring"
languages: [python]
exclude: ["**/_*.py", "**/*.pyi"]
fix: manual
fix_reason: "what the code does, in words, is the author's"
fix_exception: authored
stability: preview
ported_from: [ruff/D100, pylint/missing-module-docstring]
since: 0.5.0
---

A module whose file name does not start with `_` begins with a docstring. An empty file is not reported; a package's `__init__.py` is `py/undocumented-public-package`'s.

**Why:** A docstring is what `help()`, editors and generated documentation show; a public name without one makes its reader guess.

```codelaw
match:
  kind: module
  not: {has: {nthChild: 1, kind: expression_statement, has: {nthChild: 1, any: [{kind: string}, {kind: concatenated_string}]}}}
  has: {nthChild: 1}
message: "Add a docstring"
```

## How to fix

1. Write a docstring as the first statement: one line saying what it does, then a blank line and the details if any.

## Examples

### Python

#### Don't

```python path=store.py
import os
```

#### Do

```python path=store.py
"""Read and write the user store."""
import os
```
