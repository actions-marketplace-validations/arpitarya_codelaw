---
id: py/private-type-parameter
title: "A PEP 695 type parameter has no leading underscore"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/UP049]
since: 0.5.0
---

A type parameter of a generic class or function written with PEP 695's brackets (`class Box[_T]:`, `def first[_T](xs: list[_T]) -> _T:`), a bounded `_T: int`, `*_Ts` and `**_P` included, is named without its leading underscores: the fix renames the parameter and every use of it to `T`. A sunder or dunder name (`_T_`, `__T__`) is not private and is not reported. When the new name is already a name in the file, or would be a keyword, the finding is fixed by hand, and so is a `*_Ts` or `**_P`, which codelaw's scope model does not bind.

**Why:** A PEP 695 type parameter is already scoped to its class or function; the underscore promises a privacy it cannot add and makes the signature harder to read.

```codelaw
match:
  kind: identifier
  all:
    - regex: '^_+[A-Za-z0-9]\w*[A-Za-z0-9]$|^_+[A-Za-z0-9]$'
    - any:
        - inside: {kind: type, inside: {kind: type_parameter, inside: {any: [{kind: class_definition}, {kind: function_definition}]}}}
        - inside: {kind: splat_type, inside: {kind: type, inside: {kind: type_parameter, inside: {any: [{kind: class_definition}, {kind: function_definition}]}}}}
        - inside: {kind: type, nthChild: 1, inside: {kind: constrained_type, inside: {kind: type, inside: {kind: type_parameter, inside: {any: [{kind: class_definition}, {kind: function_definition}]}}}}}
    - not: {inside: {kind: type, has: {kind: identifier, nthChild: 2}}}
    - pattern: $T
    - any:
        - all: [{not: {inside: {kind: splat_type}}}, {not: {regex: '^_+(False|None|True|and|as|assert|async|await|break|class|continue|def|del|elif|else|except|finally|for|from|global|if|import|in|is|lambda|nonlocal|not|or|pass|raise|return|try|while|with|yield)$'}}, {pattern: $FIX}]
        - regex: ''
transform:
  NEW: { replace: { source: $T, replace: '^_+', by: '' } }
message: "Name the type parameter `$NEW`"
fix:
  - rename-symbol: { target: $FIX, to: $NEW, scope: binding }
```

## How to fix

1. Rename the type parameter without its leading underscores, with every use of it.

## Examples

### Python

#### Don't

```python
def first[_T](items: list[_T]) -> _T:
    return items[0]
```

#### Do

```python
def first[T](items: list[T]) -> T:
    return items[0]
```
