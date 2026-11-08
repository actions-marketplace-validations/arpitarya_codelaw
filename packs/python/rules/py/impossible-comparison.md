---
id: py/impossible-comparison
title: "An `and` of comparisons is not always false"
languages: [python]
fix: manual
fix_reason: "one of the two comparisons is wrong, and which one is the author's to say"
fix_exception: value
stability: preview
ported_from: [pylint/impossible-comparison]
since: 0.5.0
---

`a > b and b > a` can never be true: the two comparisons contradict each other. This rule reads two comparisons of the same two operands joined by `and`, where one says the first is larger and the other says it is smaller (`a > b and a < b`, `a >= b and b > a`, …). An operand that is a call is left alone, since it may return a different value each time; comparisons against two different numbers (`x > 10 and x < 5`) are not compared.

**Why:** A condition that is always false hides a typo in one of its halves, and the branch it guards never runs.

```codelaw
match:
  kind: boolean_operator
  any:
    - pattern: $A > $B and $B > $A
    - pattern: $A < $B and $B < $A
    - pattern: $A > $B and $A < $B
    - pattern: $A < $B and $A > $B
    - pattern: $A >= $B and $B > $A
    - pattern: $A > $B and $B >= $A
    - pattern: $A <= $B and $B < $A
    - pattern: $A < $B and $B <= $A
    - pattern: $A >= $B and $A < $B
    - pattern: $A > $B and $A <= $B
    - pattern: $A <= $B and $A > $B
    - pattern: $A < $B and $A >= $B
constraints:
  A: {not: {any: [{kind: call}, {has: {stopBy: end, kind: call}}]}}
  B: {not: {any: [{kind: call}, {has: {stopBy: end, kind: call}}]}}
message: "These comparisons contradict each other; the condition is always false"
```

## How to fix

1. Find the comparison that is the typo and correct its operator or its operand.

## Examples

### Python

#### Don't

```python
if low > high and high > low:
    swap()
```

#### Do

```python
if low > high:
    swap()
```
