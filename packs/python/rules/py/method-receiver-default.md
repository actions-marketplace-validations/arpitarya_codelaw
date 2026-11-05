---
id: py/method-receiver-default
title: "No default on self or cls"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/RUF077]
since: 0.5.0
---

A method's first parameter (`self` or `cls`) has no default value.

**Why:** The receiver is always passed; a default is dead code that confuses readers.

```codelaw
match:
  kind: default_parameter
  has: {field: name, regex: '^(self|cls)$', pattern: $P}
  inside: {kind: parameters, regex: '^\(\s*(self|cls)\s*=', inside: {kind: function_definition, inside: {kind: block, inside: {kind: class_definition}}}}
message: "`$P` is always passed; remove the default"
fix: "$P"
```

## Examples

### Python

#### Don't

```python
class A:
    def run(self=None):
        pass
```

#### Do

```python
class A:
    def run(self):
        pass
```
