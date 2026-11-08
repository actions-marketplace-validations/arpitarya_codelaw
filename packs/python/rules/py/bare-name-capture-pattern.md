---
id: py/bare-name-capture-pattern
title: "A bare name in case is the last case"
languages: [python]
fix: manual
fix_reason: "the constant the case meant is the author's"
fix_exception: value
stability: preview
ported_from: [pylint/bare-name-capture-pattern]
since: 0.5.0
---

A `case name:` — a bare name, which captures anything — is not followed by other `case` clauses.

**Why:** It matches every value, so the cases after it are unreachable; a constant meant there needs a dotted name (`Color.RED`).

```codelaw
match:
  kind: case_clause
  has: {kind: case_pattern, all: [{has: {nthChild: 1, kind: dotted_name, regex: '^[A-Za-z_]\w*$', not: {regex: '^_$'}}}, {not: {has: {nthChild: 2}}}]}
  precedes: {kind: case_clause}
message: "This case captures everything; the cases after it never match"
```

## How to fix

1. Use a dotted name for a constant (`Color.RED`), or make the capture the last case.

## Examples

### Python

#### Don't

```python
match c:
    case RED:
        pass
    case BLUE:
        pass
```

#### Do

```python
match c:
    case Color.RED:
        pass
    case Color.BLUE:
        pass
```
