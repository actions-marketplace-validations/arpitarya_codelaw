---
id: py/unrecognized-version-info-check
title: "A stub's version check has a form type checkers understand"
languages: [python]
files: ["**/*.pyi"]
fix: manual
fix_reason: "which versions the branch is for is the author's; the check has to be rewritten in a recognised form"
fix_exception: value
stability: preview
ported_from: [ruff/PYI003]
since: 0.5.0
---

In a stub, an `if` tests `sys.version_info` in a form type checkers do not evaluate. The forms they read are `sys.version_info` or `sys.version_info[:1]` / `[:2]` compared with a tuple of integers, and `sys.version_info[0]` compared with an integer. Any other slice or index (`[1:]`, `[:3]`, `[1]`), or a comparison with a list, a string, a float or `True`, is reported. Each side of an `and` or `or` is read on its own; a chained comparison is left alone, as upstream.

**Why:** A check the type checker cannot evaluate makes it consider both branches, so the stub declares names for versions that do not have them.

```codelaw
match:
  kind: comparison_operator
  any:
    - inside: {kind: if_statement, field: condition}
    - inside: {kind: boolean_operator, inside: {kind: if_statement, field: condition}}
  all:
    - has: {nthChild: 1, regex: '^sys\.version_info(\s*\[.*\])?$'}
    - not: {has: {nthChild: 3}}
    - not:
        any:
          - all:
              - has: {nthChild: 1, regex: '^sys\.version_info(\s*\[\s*:\s*[12]\s*\])?$'}
              - has: {nthChild: 2, kind: tuple, regex: '^\(\s*\d+\s*(,\s*\d+\s*)*,?\s*\)$'}
          - all:
              - has: {nthChild: 1, regex: '^sys\.version_info\s*\[\s*0\s*\]$'}
              - has: {nthChild: 2, kind: integer}
message: "Type checkers do not evaluate this `sys.version_info` check"
```

## How to fix

1. Compare `sys.version_info` with a tuple of major and minor, e.g. `sys.version_info >= (3, 10)`, or `sys.version_info[0]` with an integer.

## Examples

### Python

#### Don't

```python path=api.pyi
import sys

if sys.version_info[1] >= 10:
    def f() -> None: ...
```

#### Do

```python path=api.pyi
import sys

if sys.version_info >= (3, 10):
    def f() -> None: ...
```
