---
id: py/trailing-comma-on-bare-tuple
title: "No accidental one-element tuple"
languages: [python]
fix: manual
fix_reason: "whether a tuple was meant is the author's"
fix_exception: value
stability: preview
ported_from: [ruff/COM818]
since: 0.5.0
---

An assignment or return value is not a single item followed by a comma (`x = 1,`).

**Why:** The trailing comma makes a tuple, which is rarely meant and easy to miss.

```codelaw
match:
  kind: expression_list
  regex: ',\s*$'
  not: {has: {nthChild: 2}}
  inside: {any: [{kind: assignment}, {kind: return_statement}]}
message: "This trailing comma makes a one-element tuple"
```

## How to fix

1. Remove the comma, or write the tuple in parentheses `(x,)` if it is meant.

## Examples

### Python

#### Don't

```python
timeout = 30,
```

#### Do

```python
timeout = 30
```
