---
id: py/argument-default-in-stub
title: "A stub's defaults are simple"
languages: [python]
files: ["**/*.pyi"]
fix: auto
stability: preview
ported_from: [ruff/PYI014]
since: 0.5.0
---

A default value in a stub is a literal, a name or `...`: a call, a lambda, a comprehension or a conditional is written `...`.

**Why:** A stub only declares that a default exists; computed defaults are noise a type checker cannot use.

```codelaw
match:
  any:
    - {kind: call}
    - {kind: lambda}
    - {kind: list_comprehension}
    - {kind: dictionary_comprehension}
    - {kind: set_comprehension}
    - {kind: generator_expression}
    - {kind: conditional_expression}
  inside:
    any: [{kind: default_parameter}, {kind: typed_default_parameter}]
    field: value
message: "Write the default as `...`"
fix: "..."
```

## Examples

### Python

#### Don't

```python path=api.pyi
def connect(timeout: float = default_timeout()) -> None: ...
```

#### Do

```python path=api.pyi
def connect(timeout: float = ...) -> None: ...
```
