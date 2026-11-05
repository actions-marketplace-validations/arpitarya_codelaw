---
id: py/useless-parent-delegation
title: "No method that only calls super()"
languages: [python]
fix: suggest
fix_reason: "the method is removed; the parent's is inherited unchanged"
stability: preview
ported_from: [pylint/useless-parent-delegation]
since: 0.5.0
---

A method whose whole body calls the parent's method of the same name with the same (no) arguments is removed.

**Why:** Inheritance already does that.

```codelaw
match:
  kind: function_definition
  all:
  - has: {field: name, pattern: $M}
  - has: {field: parameters, regex: ^\(\s*self\s*\)$}
  - has:
      field: body
      regex: ^(return\s+)?super\(\)\.\w+\(\)$
      has:
        stopBy: end
        kind: attribute
        regex: ^super\(\)\.
        has: {field: attribute, pattern: $M}
  - inside:
      kind: block
      inside: {kind: class_definition}
message: This method only calls the parent's; remove it
fix: [remove]
```

## Examples

### Python

#### Don't

```python
class A(B):
    def close(self):
        super().close()
    def run(self):
        pass
```

#### Do

```python
class A(B):
    def run(self):
        pass
```
