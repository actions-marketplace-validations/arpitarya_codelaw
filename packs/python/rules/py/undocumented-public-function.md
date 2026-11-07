---
id: py/undocumented-public-function
title: "A public function has a docstring"
languages: [python]
exclude: ["**/*.pyi"]
fix: manual
fix_reason: "what the code does, in words, is the author's"
fix_exception: authored
stability: preview
ported_from: [ruff/D103, pylint/missing-function-docstring]
since: 0.5.0
---

A function at module level whose name does not start with `_` has a docstring. `@overload` and `@override` functions are not reported.

**Why:** A docstring is what `help()`, editors and generated documentation show; a public name without one makes its reader guess.

```codelaw
match:
  kind: function_definition
  has: {field: name, regex: '^[^_]'}
  any: [{inside: {kind: module}}, {inside: {kind: decorated_definition, inside: {kind: module}}}]
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
def load(path):
    return parse(read(path))
```

#### Do

```python
def load(path):
    """Read and parse the file at `path`."""
    return parse(read(path))
```
