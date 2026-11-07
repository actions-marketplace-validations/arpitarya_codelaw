---
id: py/boolean-default-value-positional-argument
title: "A parameter defaulting to True or False is keyword-only"
languages: [python]
fix: manual
fix_reason: "making the parameter keyword-only changes every call that passes it by position"
fix_exception: restructure
stability: preview
ported_from: [ruff/FBT002]
since: 0.5.0
---

A parameter whose default is `True` or `False` comes after `*`, so a caller names it. `__setitem__`, `__post_init__`, property setters and `@override` methods are not reported.

**Why:** A bare `True` at a call site says nothing about what it switches on.

```codelaw
match:
  any:
    - kind: default_parameter
    - kind: typed_default_parameter
  inside: {kind: parameters, inside: {kind: function_definition}}
  has: {field: value, regex: '^(True|False)$'}
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
def fetch(url, verify=True):
    pass
```

#### Do

```python
def fetch(url, *, verify=True):
    pass
```
