---
id: py/boolean-type-hint-positional-argument
title: "A bool parameter is keyword-only"
languages: [python]
fix: manual
fix_reason: "making the parameter keyword-only changes every call that passes it by position"
fix_exception: restructure
stability: preview
ported_from: [ruff/FBT001]
since: 0.5.0
---

A parameter annotated `bool` (or `bool | None`) comes after `*`, so a caller writes `send(msg, urgent=True)`, not `send(msg, True)`. `__setitem__`, `__post_init__`, property setters and `@override` methods are not reported.

**Why:** A bare `True` at a call site says nothing about what it switches on.

```codelaw
match:
  any:
    - kind: typed_parameter
    - kind: typed_default_parameter
  inside: {kind: parameters, inside: {kind: function_definition}}
  has: {field: type, regex: '^(bool|bool\s*\|\s*None|None\s*\|\s*bool|(typing\.)?Optional\[bool\])$'}
  all:
    - not: {follows: {stopBy: end, any: [{kind: keyword_separator}, {kind: list_splat_pattern}, {kind: typed_parameter, has: {kind: list_splat_pattern}}]}}
    - not:
        any:
          - inside: {kind: function_definition, stopBy: end, has: {field: name, regex: '^__(setitem|post_init)__$'}}
          - inside: {kind: function_definition, stopBy: end, inside: {kind: decorated_definition, has: {kind: decorator, regex: '^@((typing\.)?override|\w+\.setter)$'}}}
message: "Make the boolean parameter keyword-only"
```

## How to fix

1. Put `*` before the parameter, so it can only be passed by name, and update the calls that pass it by position.

## Examples

### Python

#### Don't

```python
def send(msg, urgent: bool):
    pass
```

#### Do

```python
def send(msg, *, urgent: bool):
    pass
```
