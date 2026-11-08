---
id: py/undocumented-public-package
title: "A public package has a docstring"
languages: [python]
files: ["**/__init__.py"]
fix: manual
fix_reason: "what the code does, in words, is the author's"
fix_exception: authored
stability: preview
ported_from: [ruff/D104]
since: 0.5.0
---

A package's `__init__.py` with any statement in it begins with a docstring.

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

```python path=users/__init__.py
from .store import Store
```

#### Do

```python path=users/__init__.py
"""The user store."""
from .store import Store
```
