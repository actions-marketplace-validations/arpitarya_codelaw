---
id: py/outdated-version-block
title: "No `sys.version_info` check the target already decides"
languages: [python]
fix: suggest
fix_reason: "the block is replaced by the branch that always runs, which is right while the target is the oldest Python the code runs on; an `elif` chain, a block on the header's line and a condition inside another expression are reported without an edit"
finding_target: { python: { from: "3.0" } }
stability: preview
ported_from: [ruff/UP036]
since: 0.5.0
---

An `if` compares `sys.version_info` with a tuple of one or two numbers (`>=`, `>`, `<`, `<=`), or `sys.version_info[0]` with a number, and the project's target settles the answer on every version it supports: `>= (3, 8)` is always true on a 3.9 target, and `< (3, 8)` always false. A check the target leaves open (`>= (3, 10)` on 3.9) is left alone. The finding is the statement; the edit keeps the branch that always runs. With no target the rule does nothing.

**Why:** A branch that never runs is dead code that still has to be read, and the version check says the project supports a Python it does not.

```codelaw
native: py/outdated-version-block
```

## How to fix

1. Keep the branch that always runs, at the indentation of the `if`, and delete the check.

## Examples

### Python

#### Don't

```python python=3.9
import sys

if sys.version_info >= (3, 8):
    from importlib import metadata
else:
    import importlib_metadata as metadata
```

#### Do

```python python=3.9
import sys

from importlib import metadata
```
