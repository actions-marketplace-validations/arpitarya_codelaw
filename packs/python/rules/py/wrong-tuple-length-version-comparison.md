---
id: py/wrong-tuple-length-version-comparison
title: "A sliced `sys.version_info` is compared with a tuple of its length"
languages: [python]
files: ["**/*.pyi"]
fix: manual
fix_reason: "whether the slice or the tuple is wrong is the author's to say"
fix_exception: value
stability: preview
ported_from: [ruff/PYI005]
since: 0.5.0
---

In a stub, `sys.version_info[:1]` is compared for equality with a tuple that does not have one item, or `sys.version_info[:2]` with one that does not have two. Such a test is never true (`==`) or always true (`!=`). Only `==` and `!=` in an `if` test are read.

**Why:** The branch under it is dead, or always taken, whatever version runs.

```codelaw
match:
  kind: comparison_operator
  any:
    - inside: {kind: if_statement, field: condition}
    - inside: {kind: boolean_operator, inside: {kind: if_statement, field: condition}}
  all:
    - has: {regex: '^(==|!=)$'}
    - not: {has: {nthChild: 3}}
    - any:
        - all:
            - has: {nthChild: 1, regex: '^sys\.version_info\s*\[\s*:\s*1\s*\]$'}
            - has: {nthChild: 2, kind: tuple, has: {nthChild: 2}}
        - all:
            - has: {nthChild: 1, regex: '^sys\.version_info\s*\[\s*:\s*2\s*\]$'}
            - has:
                nthChild: 2
                kind: tuple
                any:
                  - not: {has: {nthChild: 2}}
                  - has: {nthChild: 3}
message: "The slice and the tuple have different lengths; this is never equal"
```

## How to fix

1. Make the slice and the tuple the same length: `sys.version_info[:2] == (3, 10)`.

## Examples

### Python

#### Don't

```python path=api.pyi
import sys

if sys.version_info[:1] == (3, 10):
    def f() -> None: ...
```

#### Do

```python path=api.pyi
import sys

if sys.version_info[:2] == (3, 10):
    def f() -> None: ...
```
