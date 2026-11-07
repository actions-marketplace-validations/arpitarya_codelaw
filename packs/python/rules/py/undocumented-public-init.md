---
id: py/undocumented-public-init
title: "A public class's __init__ has a docstring"
languages: [python]
exclude: ["**/*.pyi"]
fix: manual
fix_reason: "what the code does, in words, is the author's"
fix_exception: authored
stability: preview
ported_from: [ruff/D107]
since: 0.5.0
---

The `__init__` of a public class has a docstring.

**Why:** A docstring is what `help()`, editors and generated documentation show; a public name without one makes its reader guess.

```codelaw
match:
  kind: function_definition
  has: {field: name, regex: '^__init__$'}
  any:
    - inside: {kind: block, inside: {kind: class_definition, has: {field: name, regex: '^[^_]'}, any: [{inside: {kind: module}}, {inside: {kind: decorated_definition, inside: {kind: module}}}]}}
    - inside: {kind: decorated_definition, inside: {kind: block, inside: {kind: class_definition, has: {field: name, regex: '^[^_]'}, any: [{inside: {kind: module}}, {inside: {kind: decorated_definition, inside: {kind: module}}}]}}}
  all:
    - not: {has: {field: body, has: {nthChild: 1, kind: expression_statement, has: {nthChild: 1, any: [{kind: string}, {kind: concatenated_string}]}}}}
    - not: {inside: {kind: decorated_definition, has: {kind: decorator, regex: '^@((typing\.)?(overload|override)|\w+\.(setter|deleter))$'}}}
message: "Add a docstring"
```

## How to fix

1. Write a docstring as the first statement: one line saying what it does, then a blank line and the details if any.

## Examples

### Python

#### Don't

```python
class Store:
    """The users, keyed by id."""

    def __init__(self, path):
        self.path = path
```

#### Do

```python
class Store:
    """The users, keyed by id."""

    def __init__(self, path):
        """Open the store kept at `path`."""
        self.path = path
```
