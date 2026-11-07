---
id: py/invalid-function-name
title: "Function names are lowercase"
languages: [python]
fix: manual
fix_reason: "its callers, in this file and in others, use the name; renaming it is a change across the code base"
fix_exception: restructure
stability: preview
ported_from: [ruff/N802, pylint/invalid-name]
since: 0.5.0
---

A function or method name has no capital letter: `load_user`, not `loadUser`. unittest's hooks (`setUp`, `tearDown`, `setUpClass`, `tearDownClass`, `setUpModule`, `tearDownModule`, `asyncSetUp`, `asyncTearDown`, `setUpTestData`) and a method marked `@override` are not reported.

**Why:** PEP 8 names functions in lowercase with underscores, so a function reads apart from a class.

```codelaw
match:
  kind: identifier
  pattern: $N
  regex: '[A-Z]'
  inside: {kind: function_definition, field: name}
  all:
    - not: {regex: '^(setUp|tearDown|setUpClass|tearDownClass|setUpModule|tearDownModule|asyncSetUp|asyncTearDown|setUpTestData|failureException|longMessage|maxDiff)$'}
    - not:
        inside:
          kind: function_definition
          inside: {kind: decorated_definition, has: {kind: decorator, regex: '^@(typing\.)?override$'}}
message: "Name the function in lowercase: `$N`"
```

## How to fix

1. Rename the function in snake_case (`loadUser` → `load_user`) and update every caller, in this file and the modules that import it.

## Examples

### Python

#### Don't

```python
def loadUser(user_id):
    return db.get(user_id)
```

#### Do

```python
def load_user(user_id):
    return db.get(user_id)
```
