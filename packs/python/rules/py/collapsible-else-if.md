---
id: py/collapsible-else-if
title: "elif, not else: if"
languages: [python]
fix: suggest
fix_reason: "the inner `if` moves up one level and its body is re-indented"
stability: preview
ported_from: [ruff/PLR5501, pylint/else-if-used]
since: 0.5.0
---

An `else:` whose only statement is an `if` (with a one-statement body and no `else`) is written `elif`.

**Why:** `elif` flattens the nesting.

```codelaw
match:
  kind: else_clause
  has:
    field: body
    not: {has: {nthChild: 2}}
    has: {kind: if_statement, pattern: "if $C:\n    $S", not: {has: {any: [{kind: else_clause}, {kind: elif_clause}]}}}
  inside: {kind: if_statement, inside: {kind: module}}
message: "Write `elif $C:`"
fix: "elif $C:\n    $S"
```

## Examples

### Python

#### Don't

```python
if a:
    x()
else:
    if b:
        y()
```

#### Do

```python
if a:
    x()
elif b:
    y()
```
