---
id: py/no-self-argument
title: "A method's first parameter is self"
languages: [python]
fix: suggest
fix_reason: "the parameter and its uses in the method are renamed"
stability: preview
ported_from: [ruff/N805, pylint/no-self-argument]
since: 0.5.0
---

The first parameter of a plain method (no `@staticmethod` or `@classmethod`) is named `self`.

**Why:** Python passes the instance there; another name hides that.

```codelaw
match:
  kind: identifier
  pattern: $P
  all:
  - not: {regex: ^(self|_self|cls|mcs|klass|_)$}
  - not:
      follows: {stopBy: end, kind: identifier}
  - inside:
      kind: parameters
      regex: ^\(\s*\w+
      inside:
        kind: function_definition
        all:
        - not:
            has: {field: name, regex: ^(__new__|__init_subclass__|__class_getitem__)$}
        - not:
            inside: {kind: decorated_definition}
        - inside:
            kind: block
            inside:
              kind: class_definition
              not:
                has: {field: superclasses, regex: \btype\b}
message: Name the first parameter `self`
fix:
- rename-symbol-local: {target: $P, to: self}
```

## Examples

### Python

#### Don't

```python
class A:
    def size(this):
        return this.n
```

#### Do

```python
class A:
    def size(self):
        return self.n
```
