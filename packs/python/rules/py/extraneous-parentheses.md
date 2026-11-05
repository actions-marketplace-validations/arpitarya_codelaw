---
id: py/extraneous-parentheses
title: "No double parentheses in a call"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/UP034]
since: 0.5.0
---

`print((x))` is written `print(x)`.

**Why:** The inner parentheses group nothing.

```codelaw
match:
  kind: argument_list
  has: {kind: parenthesized_expression, pattern: ($X), not: {has: {any: [{kind: generator_expression}, {kind: yield}, {kind: named_expression}]}}}
  not: {has: {nthChild: 2}}
  regex: '^\(\(.*\)\)$'
message: "Remove the extra parentheses"
fix: "($X)"
```

## Examples

### Python

#### Don't

```python
print((total))
```

#### Do

```python
print(total)
```
