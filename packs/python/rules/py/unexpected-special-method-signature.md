---
id: py/unexpected-special-method-signature
title: "Special methods take the parameters Python passes"
languages: [python]
fix: manual
fix_reason: "which parameters to keep is the author's"
fix_exception: value
stability: preview
ported_from: [ruff/PLE0302]
since: 0.5.0
---

`__len__`, `__bool__`, `__hash__`, `__str__`, `__repr__`, `__iter__`, `__next__`, `__enter__`, `__aenter__`, `__index__`, `__int__`, `__float__`, `__neg__`, `__pos__`, `__abs__` and `__invert__` take only `self`; `__eq__`, `__lt__` and the other comparisons take one more. (Ruff checks every special method's arity; the common ones are covered.)

**Why:** Python calls them with a fixed number of arguments; any other signature raises TypeError at the call.

```codelaw
match:
  kind: function_definition
  any:
    - all:
        - has: {field: name, regex: '^__(len|bool|hash|str|repr|iter|next|enter|aenter|index|int|float|neg|pos|abs|invert|aiter|anext|bytes|complex)__$'}
        - has: {field: parameters, regex: '^\(\s*\w+\s*,\s*[A-Za-z_]', not: {regex: '\*'}}
    - all:
        - has: {field: name, regex: '^__(eq|ne|lt|le|gt|ge|contains|getitem|delitem)__$'}
        - has: {field: parameters, any: [{regex: '^\(\s*\w+\s*\)$'}, {regex: '^\(\s*\w+\s*,[^,=*]+,[^=*]+\)$'}]}
  inside: {kind: block, inside: {kind: class_definition}}
message: "This special method has the wrong number of parameters"
```

## How to fix

1. Give the method the parameters Python passes: `def __len__(self)`, `def __eq__(self, other)`.

## Examples

### Python

#### Don't

```python
class A:
    def __len__(self, extra):
        return 0
```

#### Do

```python
class A:
    def __len__(self):
        return 0
```
