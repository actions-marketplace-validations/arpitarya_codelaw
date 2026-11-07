---
id: py/redefined-outer-name
title: "A function does not reuse a name from the scope around it"
languages: [python]
fix: manual
fix_reason: "the new name is the author's"
fix_exception: value
stability: preview
ported_from: [pylint/redefined-outer-name]
since: 0.5.0
---

A variable or parameter inside a function does not take the name of a variable, function or import of an enclosing scope.

**Why:** Inside the function the outer name is hidden, and an edit that meant the outer one gets the inner.

```codelaw
match:
  kind: identifier
  pattern: $N
  any:
    - inside: {kind: parameters}
    - inside: {kind: default_parameter, field: name}
    - inside: {kind: typed_parameter}
    - inside: {kind: typed_default_parameter, field: name}
    - inside: {kind: assignment, field: left}
    - inside: {kind: for_statement, field: left}
  inside: {kind: function_definition, stopBy: end}
where:
  $N: shadowed
message: "`$N` hides the outer name of that name"
```

## How to fix

1. Rename the inner name so it no longer hides the outer one.

## Examples

### Python

#### Don't

```python
config = load()


def run(config):
    return config
```

#### Do

```python
config = load()


def run(settings):
    return settings
```
