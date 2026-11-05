---
id: py/bad-mcs-method-argument
title: "A metaclass method's first parameter is cls"
languages: [python]
fix: suggest
fix_reason: "the parameter and its uses in the method are renamed"
stability: preview
ported_from: [pylint/bad-mcs-method-argument]
since: 0.5.0
---

In a metaclass (a class deriving from `type`), a plain method's first parameter is named `cls`.

**Why:** The instance of a metaclass is a class; `cls` says so.

```codelaw
match:
  kind: identifier
  pattern: $P
  all:
  - not: {regex: ^(cls|_cls|mcs|metacls)$}
  - not:
      follows: {stopBy: end, kind: identifier}
  - inside:
      kind: parameters
      regex: ^\(\s*\w+
      inside:
        kind: function_definition
        all:
        - not:
            has: {field: name, regex: ^(__new__|__prepare__)$}
        - not:
            inside: {kind: decorated_definition}
        - inside:
            kind: block
            inside:
              kind: class_definition
              has: {field: superclasses, regex: ^\(\s*type\s*\)$}
message: Name the first parameter `cls`
fix:
- rename-symbol-local: {target: $P, to: cls}
```

## Examples

### Python

#### Don't

```python
class Meta(type):
    def describe(self):
        return self.__name__
```

#### Do

```python
class Meta(type):
    def describe(cls):
        return cls.__name__
```
