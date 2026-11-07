---
id: py/undocumented-public-nested-class
title: "A public nested class has a docstring"
languages: [python]
exclude: ["**/*.pyi"]
fix: manual
fix_reason: "what the code does, in words, is the author's"
fix_exception: authored
stability: preview
ported_from: [ruff/D106]
since: 0.5.0
---

A class defined in a public class's body, whose name does not start with `_`, has a docstring.

**Why:** A docstring is what `help()`, editors and generated documentation show; a public name without one makes its reader guess.

```codelaw
match:
  kind: class_definition
  not: {has: {field: body, has: {nthChild: 1, kind: expression_statement, has: {nthChild: 1, any: [{kind: string}, {kind: concatenated_string}]}}}}
  has: {field: name, regex: '^[^_]'}
  any:
    - inside: {kind: block, inside: {kind: class_definition, has: {field: name, regex: '^[^_]'}, any: [{inside: {kind: module}}, {inside: {kind: decorated_definition, inside: {kind: module}}}]}}
    - inside: {kind: decorated_definition, inside: {kind: block, inside: {kind: class_definition, has: {field: name, regex: '^[^_]'}, any: [{inside: {kind: module}}, {inside: {kind: decorated_definition, inside: {kind: module}}}]}}}
message: "Add a docstring"
```

## How to fix

1. Write a docstring as the first statement: one line saying what it does, then a blank line and the details if any.

## Examples

### Python

#### Don't

```python
class Form:
    class Meta:
        model = User
```

#### Do

```python
class Form:
    class Meta:
        """How the form maps onto the model."""

        model = User
```
