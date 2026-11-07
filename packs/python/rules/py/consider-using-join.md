---
id: py/consider-using-join
title: "str.join, not += in a loop"
languages: [python]
fix: manual
fix_reason: "the loop and its accumulator become one expression"
fix_exception: restructure
stability: preview
ported_from: [pylint/consider-using-join]
since: 0.5.0
---

A loop whose body is only `s += item`, where `s` was assigned `""` just before, is written `s = "".join(...)`.

**Why:** Each `+=` copies the string built so far; `join` builds it once.

```codelaw
match:
  kind: for_statement
  all:
    - has: {field: body, all: [{has: {nthChild: 1, kind: expression_statement, has: {kind: augmented_assignment, pattern: '$S += $E'}}}, {not: {has: {nthChild: 2}}}]}
    - follows: {kind: expression_statement, has: {kind: assignment, pattern: '$S = ""'}}
message: "Build the string with `str.join`"
```

## How to fix

1. Replace the accumulator and the loop with `s = "".join(<expr> for <item> in <items>)`.

## Examples

### Python

#### Don't

```python
s = ""
for w in words:
    s += w
```

#### Do

```python
s = "".join(words)
```
