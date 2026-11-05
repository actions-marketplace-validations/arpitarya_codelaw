---
id: py/unused-method-argument
title: "No unused method parameter"
languages: [python]
fix: manual
fix_reason: "removing or renaming a parameter changes every caller"
fix_exception: restructure
stability: preview
ported_from: [ruff/ARG002]
since: 0.5.0
---

A parameter of an instance method (other than `self`) is used in its body. Names starting with `_`, stub bodies, decorated methods and dunder methods (whose signature Python fixes) are left alone.

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
    inside: {kind: block, inside: {kind: class_definition}}
    not: {has: {field: name, regex: '^__\w+__$'}}
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
class A:
    def area(self, unit):
        return self.w * self.h
```

#### Do

```python
class A:
    def area(self):
        return self.w * self.h
```
