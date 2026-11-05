---
id: py/unnecessary-class-parentheses
title: "class A:, not class A():"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/UP039]
since: 0.5.0
---

A class with no bases is written without empty parentheses.

**Why:** The parentheses say nothing.

```codelaw
match:
  kind: argument_list
  regex: '^\(\s*\)$'
  inside: {kind: class_definition}
message: "Remove the empty parentheses"
fix: ""
```

## Examples

### Python

#### Don't

```python
class A():
    pass
```

#### Do

```python
class A:
    pass
```
