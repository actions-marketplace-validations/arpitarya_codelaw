---
id: py/bad-mcs-classmethod-argument
title: "A metaclass classmethod's first parameter is mcs"
languages: [python]
fix: suggest
fix_reason: "the parameter and its uses in the method are renamed"
stability: preview
ported_from: [pylint/bad-mcs-classmethod-argument]
since: 0.5.0
---

In a metaclass, a `@classmethod`'s first parameter is named `mcs`.

**Why:** It receives the metaclass itself; `mcs` keeps it apart from the `cls` of its instances.

```codelaw
match:
  kind: identifier
  pattern: $P
  all:
  - not: {regex: ^(mcs|_mcs|metacls)$}
  - not:
      follows: {stopBy: end, kind: identifier}
  - inside:
      kind: parameters
      regex: ^\(\s*\w+
      inside:
        kind: function_definition
        inside:
          kind: decorated_definition
          has: {kind: decorator, regex: ^@classmethod$}
          inside:
            kind: block
            inside:
              kind: class_definition
              has: {field: superclasses, regex: ^\(\s*type\s*\)$}
message: Name the first parameter `mcs`
fix:
- rename-symbol-local: {target: $P, to: mcs}
```

## Examples

### Python

#### Don't

```python
class Meta(type):
    @classmethod
    def build(cls):
        return cls
```

#### Do

```python
class Meta(type):
    @classmethod
    def build(mcs):
        return mcs
```
