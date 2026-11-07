---
id: py/any-type
title: "No Any in a function signature"
languages: [python]
fix: manual
fix_reason: "the type that fits is the author's"
fix_exception: value
stability: preview
ported_from: [ruff/ANN401]
since: 0.5.0
---

A parameter or return type is not annotated `Any` (or `typing.Any`). `*args: Any` and `**kwargs: Any` are not reported.

**Why:** `Any` turns type checking off for everything the value touches.

```codelaw
match:
  kind: type
  regex: '^(typing\.)?Any$'
  any:
    - inside: {kind: typed_parameter, not: {has: {any: [{kind: list_splat_pattern}, {kind: dictionary_splat_pattern}]}}, inside: {kind: parameters, inside: {kind: function_definition}}}
    - inside: {kind: typed_default_parameter, inside: {kind: parameters, inside: {kind: function_definition}}}
    - inside: {kind: function_definition, field: return_type}
message: "Name the type instead of `Any`"
```

## How to fix

1. Replace `Any` with the type the value has, `object` if it can be anything, or a `TypeVar` if it is passed through.

## Examples

### Python

#### Don't

```python
def parse(raw: Any) -> Any:
    pass
```

#### Do

```python
def parse(raw: str) -> dict[str, int]:
    pass
```
