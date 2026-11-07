---
id: py/match-class-bind-self
title: "A builtin class pattern binds its subject with `as`"
languages: [python]
fix: auto
stability: preview
ported_from: [pylint/match-class-bind-self]
since: 0.5.0
---

`case int(n):` matches an `int` and binds the subject itself to `n`, because the builtins `bool`, `bytearray`, `bytes`, `dict`, `float`, `frozenset`, `int`, `list`, `set`, `str` and `tuple` match their one positional sub-pattern against the whole subject. `case int() as n:` says that plainly and does the same.

**Why:** `int(n)` reads like a pattern that pulls a value out of the object; nothing is pulled out.

```codelaw
match:
  kind: class_pattern
  all:
    - has:
        nthChild: 1
        kind: dotted_name
        pattern: $C
        regex: '^(bool|bytearray|bytes|dict|float|frozenset|int|list|set|str|tuple)$'
    - has:
        nthChild: 2
        kind: case_pattern
        has: {kind: dotted_name, regex: '^[A-Za-z_]\w*$', not: {regex: '^_$'}, pattern: $N}
        not: {has: {kind: keyword_pattern}}
    - not: {has: {nthChild: 3}}
message: "Write `$C() as $N`: the builtin binds the whole subject"
fix: "$C() as $N"
```

## Examples

### Python

#### Don't

```python
match value:
    case int(n):
        handle(n)
```

#### Do

```python
match value:
    case int() as n:
        handle(n)
```
