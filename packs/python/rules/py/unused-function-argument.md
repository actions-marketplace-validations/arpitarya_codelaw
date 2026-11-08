---
id: py/unused-function-argument
title: "No unused function parameter"
languages: [python]
fix: manual
fix_reason: "removing or renaming a parameter changes every caller"
fix_exception: restructure
stability: preview
ported_from: [ruff/ARG001, pylint/unused-argument]
since: 0.5.0
---

A parameter of a module-level function is used in its body. Names starting with `_`, stub bodies (`...`, `pass`, `raise NotImplementedError`) and decorated functions (which may implement an interface) are left alone.

**Why:** An unused parameter is dead input that callers must still pass.

```codelaw
match:
  kind: identifier
  regex: '^[a-zA-Z]\w*$'
  pattern: $P
  any:
    - inside: {kind: parameters}
    - inside: {any: [{kind: typed_parameter}, {kind: default_parameter}, {kind: typed_default_parameter}], inside: {kind: parameters}}
  not: {inside: {kind: default_parameter, has: {field: value, stopBy: end, pattern: $P}}}
  inside:
    kind: function_definition
    stopBy: end
    inside: {kind: module}
    has: {field: body, kind: block, not: {regex: '^(\.\.\.|pass|raise\s+NotImplementedError.*|("""[\s\S]*"""|''''''[\s\S]*'''''')(\s+(\.\.\.|pass))?)$'}}
where:
  $P: unused
message: "`$P` is never used"
```

## How to fix

1. Remove the parameter and update the callers, or prefix it with `_` if an interface requires it.

## Examples

### Python

#### Don't

```python
def greet(name, loud):
    return "hi " + name
```

#### Do

```python
def greet(name):
    return "hi " + name
```
