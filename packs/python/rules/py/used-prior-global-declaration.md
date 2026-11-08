---
id: py/used-prior-global-declaration
title: "A function's `global` comes before the name is used"
languages: [python]
fix: manual
fix_reason: "moving the declaration to the top of the function changes which variable the earlier lines read, which is what the author must check"
fix_exception: restructure
stability: preview
ported_from: [pylint/used-prior-global-declaration]
since: 0.5.0
---

A function names a variable in an earlier statement and then declares it `global`. Python refuses the module when it compiles it: the declaration has to come first. Only statements before the `global` in the same block are read; a use inside a nested function does not count. A `global` that names several variables is read for its first.

**Why:** The module does not import at all.

```codelaw
match:
  kind: global_statement
  has: {kind: identifier, pattern: $N}
  inside: {stopBy: end, kind: function_definition}
  follows:
    stopBy: end
    not: {any: [{kind: function_definition}, {kind: class_definition}, {kind: decorated_definition}]}
    has:
      stopBy: {any: [{kind: function_definition}, {kind: class_definition}, {kind: lambda}]}
      kind: identifier
      pattern: $N
message: "`$N` is used before its `global` declaration"
```

## How to fix

1. Move the `global` statement to the top of the function, and check that the lines before it meant the module's variable.

## Examples

### Python

#### Don't

```python
def bump():
    print(counter)
    global counter
    counter += 1
```

#### Do

```python
def bump():
    global counter
    print(counter)
    counter += 1
```
