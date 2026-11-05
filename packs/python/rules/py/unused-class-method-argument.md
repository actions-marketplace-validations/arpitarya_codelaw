---
id: py/unused-class-method-argument
title: "No unused classmethod parameter"
languages: [python]
fix: manual
fix_reason: "removing or renaming a parameter changes every caller"
fix_exception: restructure
stability: preview
ported_from: [ruff/ARG003]
since: 0.5.0
---

A parameter of a `@classmethod` (other than `cls`) is used in its body; `_`-prefixed names and stub bodies are left alone.

**Why:** An unused parameter is dead input that callers must still pass.

```codelaw
match:
  kind: identifier
  regex: '^[a-zA-Z]\w*$'
  not: {regex: '^(self|cls)$'}
  pattern: $P
  any:
    - inside: {kind: parameters}
    - inside: {any: [{kind: typed_parameter}, {kind: default_parameter}, {kind: typed_default_parameter}], inside: {kind: parameters}}
  inside:
    kind: function_definition
    stopBy: end
    inside: {kind: decorated_definition, has: {kind: decorator, regex: '^@classmethod$'}, inside: {kind: block, inside: {kind: class_definition}}}
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
    @classmethod
    def make(cls, size):
        return cls()
```

#### Do

```python
class A:
    @classmethod
    def make(cls):
        return cls()
```
