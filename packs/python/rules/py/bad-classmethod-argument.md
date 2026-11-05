---
id: py/bad-classmethod-argument
title: "A classmethod's first parameter is cls"
languages: [python]
fix: suggest
fix_reason: "the parameter and its uses in the method are renamed"
stability: preview
ported_from: [ruff/N804, pylint/bad-classmethod-argument]
since: 0.5.0
---

The first parameter of a `@classmethod` is named `cls`.

**Why:** The convention tells readers what the parameter holds.

```codelaw
match:
  kind: identifier
  pattern: $P
  all:
  - not: {regex: ^(cls|_cls|klass|mcs|metacls|self)$}
  - inside:
      kind: parameters
      regex: ^\(\s*\w+
      inside:
        kind: function_definition
        inside:
          kind: decorated_definition
          has: {kind: decorator, regex: ^@classmethod$}
  - not:
      follows: {stopBy: end, kind: identifier}
message: Name the first parameter `cls`
fix:
- rename-symbol-local: {target: $P, to: cls}
```

## Examples

### Python

#### Don't

```python
class A:
    @classmethod
    def make(klass2):
        return klass2()
```

#### Do

```python
class A:
    @classmethod
    def make(cls):
        return cls()
```
