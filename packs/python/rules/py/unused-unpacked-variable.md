---
id: py/unused-unpacked-variable
title: "Unused names from unpacking start with _"
languages: [python]
fix: suggest
fix_reason: "the name changes; review that nothing else reads it"
stability: preview
ported_from: [ruff/RUF059]
since: 0.5.0
---

A name bound by tuple unpacking in a function and never read is prefixed `_`.

**Why:** The underscore says the value is deliberately ignored.

```codelaw
match:
  kind: identifier
  regex: '^[a-z][a-zA-Z0-9_]*$'
  pattern: $N
  inside: {any: [{kind: pattern_list}, {kind: tuple_pattern}], inside: {kind: assignment, inside: {kind: expression_statement, inside: {kind: block, inside: {kind: function_definition}}}}}
where:
  $N: unused
message: "`$N` is never used; name it `_$N`"
fix: "_$N"
```

## Examples

### Python

#### Don't

```python
def f(pair):
    key, value = pair
    return key
```

#### Do

```python
def f(pair):
    key, _value = pair
    return key
```
