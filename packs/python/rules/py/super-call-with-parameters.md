---
id: py/super-call-with-parameters
title: "super(), not super(Class, self)"
languages: [python]
fix: auto
fix_reason: "auto inside a method of the named class, where the two forms are the same"
stability: preview
ported_from: [ruff/UP008, pylint/super-with-arguments]
since: 0.5.0
---

`super(MyClass, self)` inside `MyClass`'s method is written `super()`.

**Why:** The arguments repeat what Python already knows, and break if the class is renamed.

```codelaw
match:
  kind: call
  pattern: super($C, $S)
  inside:
    kind: function_definition
    stopBy: end
    has: {field: parameters, regex: '^\(\s*(self|cls)\b'}
    inside: {kind: block, inside: {kind: class_definition, has: {field: name, pattern: $C}}}
constraints:
  S: {regex: '^(self|cls)$'}
message: "Write `super()`"
fix: "super()"
```

## Examples

### Python

#### Don't

```python
class A(B):
    def __init__(self):
        super(A, self).__init__()
```

#### Do

```python
class A(B):
    def __init__(self):
        super().__init__()
```
