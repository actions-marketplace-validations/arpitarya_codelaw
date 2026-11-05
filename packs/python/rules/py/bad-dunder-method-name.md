---
id: py/bad-dunder-method-name
title: "No made-up dunder method names"
languages: [python]
fix: manual
fix_reason: "the intended name is the author's"
fix_exception: value
stability: preview
ported_from: [ruff/PLW3201]
since: 0.5.0
---

A method named `__like_this__` is one of Python's special methods.

**Why:** An unknown dunder is never called by Python; it is usually a misspelling (`__inti__`).

```codelaw
match:
  kind: function_definition
  has:
    field: name
    regex: '^__\w+__$'
    not: {regex: '^__(init|new|del|repr|str|bytes|format|lt|le|eq|ne|gt|ge|hash|bool|getattr|getattribute|setattr|delattr|dir|get|set|delete|set_name|init_subclass|class_getitem|instancecheck|subclasscheck|call|len|length_hint|getitem|setitem|delitem|missing|iter|reversed|contains|add|sub|mul|matmul|truediv|floordiv|mod|divmod|pow|lshift|rshift|and|xor|or|radd|rsub|rmul|rmatmul|rtruediv|rfloordiv|rmod|rdivmod|rpow|rlshift|rrshift|rand|rxor|ror|iadd|isub|imul|imatmul|itruediv|ifloordiv|imod|ipow|ilshift|irshift|iand|ixor|ior|neg|pos|abs|invert|complex|int|float|index|round|trunc|floor|ceil|enter|exit|await|aiter|anext|aenter|aexit|buffer|release_buffer|copy|deepcopy|reduce|reduce_ex|getnewargs|getnewargs_ex|getstate|setstate|post_init|fspath|sizeof|subclasshook|prepare|mro_entries|instancecheck|replace|match_args|slots|annotate|attrs_post_init|attrs_pre_init|attrs_init|getinitargs|unicode|next|html|json|rich|rich_console|rich_measure|rich_repr|clause_element|array|array_interface|array_ufunc|array_function|fspath|class_getitem|nonzero|div|rdiv|idiv|cmp|coerce|long|oct|hex|getslice|setslice|delslice)__$'}
  inside: {kind: block, inside: {kind: class_definition}}
message: "This is not a special method Python calls; check the name"
```

## How to fix

1. Rename it to the special method you meant, or drop the double underscores.

## Examples

### Python

#### Don't

```python
class A:
    def __inti__(self):
        pass
```

#### Do

```python
class A:
    def __init__(self):
        pass
```
