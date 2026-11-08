---
id: py/magic-value-comparison
title: "A comparison names its number"
languages: [python]
fix: manual
fix_reason: "the constant's name says what the number means, which only the author knows"
fix_exception: value
stability: preview
ported_from: [ruff/PLR2004, pylint/magic-value-comparison]
since: 0.5.0
---

A comparison has a number literal on one side, other than `0`, `1`, `0.0` and `1.0` (with or without a sign), and something that is not a literal on the other: `if retries > 3:`. Give the number a name. Strings and bytes are allowed, as upstream allows them by default; a comparison of two literals is another rule's (`py/comparison-of-constant`), and one against `sys.version_info`, `sys.version` or `sys.implementation.version` is left alone.

**Why:** A bare number in a test says nothing about why that value; a named constant says it once and is changed in one place.

```codelaw
utils:
  number: {any: [{kind: integer}, {kind: float}]}
  magic:
    any:
      - {matches: number, not: {regex: '^(0|1|0\.0|1\.0)$'}, inside: {kind: comparison_operator}}
      - kind: unary_operator
        has: {matches: number}
        not: {regex: '^[-+~]\s*(0|1|0\.0|1\.0)$'}
        inside: {kind: comparison_operator}
match:
  matches: magic
  inside:
    kind: comparison_operator
    all:
      - not: {has: {regex: '^sys\.(version|version_info|implementation\.version)\b'}}
      - not:
          all:
            - has: {nthChild: 1, any: [{matches: number}, {kind: unary_operator, has: {matches: number}}, {kind: string}, {kind: "true"}, {kind: "false"}, {kind: none}]}
            - has: {nthChild: 2, any: [{matches: number}, {kind: unary_operator, has: {matches: number}}, {kind: string}, {kind: "true"}, {kind: "false"}, {kind: none}]}
message: "Name this number as a constant"
```

## How to fix

1. Define a module-level constant with a name that says what the number is, and compare with it.

## Examples

### Python

#### Don't

```python
if attempts > 3:
    give_up()
```

#### Do

```python
MAX_ATTEMPTS = 3

if attempts > MAX_ATTEMPTS:
    give_up()
```
