---
id: py/pointless-string-statement
title: "No string as a statement"
languages: [python]
fix: suggest
fix_reason: "the string is removed; turn it into a comment instead if it carried a note"
stability: preview
ported_from: [pylint/pointless-string-statement]
since: 0.5.0
---

A string literal that is not a docstring (not the first statement of a module, class or function, nor right after an assignment as an attribute docstring) is removed.

**Why:** It does nothing; a comment says it.

```codelaw
match:
  kind: expression_statement
  has:
    kind: string
    nthChild: 1
    not: {regex: '^[rRuU]?[fF]'}
  not:
    has: {nthChild: 2}
  follows:
    kind: expression_statement
    all:
    - not:
        has: {kind: assignment}
    - not:
        has: {kind: string}
message: This string does nothing; use a comment
fix: [remove]
```

## Examples

### Python

#### Don't

```python
print(1)
"a note"
print(2)
```

#### Do

```python
print(1)
print(2)
```
