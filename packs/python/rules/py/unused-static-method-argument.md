---
id: py/unused-static-method-argument
title: "No unused staticmethod parameter"
languages: [python]
fix: manual
fix_reason: "removing or renaming a parameter changes every caller"
fix_exception: restructure
stability: preview
ported_from: [ruff/ARG004]
since: 0.5.0
---

A parameter of a `@staticmethod` is used in its body; `_`-prefixed names and stub bodies are left alone.

**Why:** An unused parameter is dead input that callers must still pass.

```codelaw
match:
  kind: identifier
  regex: '^[a-zA-Z]\w*$'
  pattern: $P
  any:
    - inside: {kind: parameters}
    - inside: {any: [{kind: typed_parameter}, {kind: default_parameter}, {kind: typed_default_parameter}], inside: {kind: parameters}}
  inside:
    kind: function_definition
    stopBy: end
    inside: {kind: decorated_definition, has: {kind: decorator, regex: '^@staticmethod$'}, inside: {kind: block, inside: {kind: class_definition}}}
    has: {field: body, kind: block, not: {regex: '^(\.\.\.|pass|raise\s+NotImplementedError.*)$'}}
where:
  $P: unused
message: "`$P` is never used"
```

## How to fix

1. Remove the parameter and update the callers, or prefix it with `_`.

## Examples

### Python

#### Don't

```python
class A:
    @staticmethod
    def double(x, y):
        return x * 2
```

#### Do

```python
class A:
    @staticmethod
    def double(x):
        return x * 2
```
