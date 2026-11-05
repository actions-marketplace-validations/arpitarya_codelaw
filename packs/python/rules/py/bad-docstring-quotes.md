---
id: py/bad-docstring-quotes
title: "Docstrings use triple double quotes"
languages: [python]
fix: auto
fix_reason: "auto when the docstring holds no double quote, so changing the delimiters cannot end it early"
stability: preview
ported_from: [ruff/D300, pylint/bad-docstring-quotes]
since: 0.5.0
---

A docstring written with `'''` is written with `"""` (PEP 257).

**Why:** One quoting style for every docstring.

```codelaw
match:
  kind: string
  regex: "^'''"
  has: {kind: string_content, pattern: $C, not: {regex: '"'}}
  not: {has: {kind: escape_sequence}}
  inside: {kind: expression_statement, nthChild: 1, inside: {kind: block, inside: {any: [{kind: function_definition}, {kind: class_definition}]}}}
message: "Use triple double quotes"
fix: '"""$C"""'
```

## Examples

### Python

#### Don't

```python
def f():
    '''Do it.'''
```

#### Do

```python
def f():
    """Do it."""
```
