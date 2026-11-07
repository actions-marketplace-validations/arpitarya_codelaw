---
id: py/non-pep646-unpack
title: "*Ts, not Unpack[Ts]"
languages: [python]
fix: auto
fix_target: { python: "3.11" }
stability: preview
ported_from: [ruff/UP044]
since: 0.5.0
---

`*args: Unpack[Ts]` is written `*args: *Ts`, and `tuple[int, Unpack[Ts]]` `tuple[int, *Ts]` (Python 3.11+, PEP 646). `**kwargs: Unpack[Options]` (PEP 692) is not touched, nor `typing.Unpack[...]` written with its module.

**Why:** The star is the syntax PEP 646 added for the same thing.

```codelaw
match:
  kind: generic_type
  pattern: $U
  all:
    - has: {nthChild: 1, kind: identifier, regex: '^Unpack$'}
    - has: {kind: type_parameter, all: [{has: {nthChild: 1, kind: type, pattern: $T}}, {not: {has: {nthChild: 2}}}]}
    - any:
        - inside: {kind: type, inside: {kind: typed_parameter, has: {kind: list_splat_pattern}}}
        - inside: {kind: type, inside: {kind: type_parameter}}
message: "Write `*$T`"
fix:
  - replace: {target: $U, with: "*$T"}
```

## Examples

### Python

#### Don't

```python
def call(*args: Unpack[Ts]) -> tuple[int, Unpack[Ts]]:
    pass
```

#### Do

```python
def call(*args: *Ts) -> tuple[int, *Ts]:
    pass
```
