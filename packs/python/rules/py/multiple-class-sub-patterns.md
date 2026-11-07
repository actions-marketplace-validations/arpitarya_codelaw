---
id: py/multiple-class-sub-patterns
title: "A class pattern names each attribute once"
languages: [python]
fix: manual
fix_reason: "which of the two sub-patterns the attribute should match is the author's"
fix_exception: value
stability: preview
ported_from: [pylint/multiple-class-sub-patterns]
since: 0.5.0
---

`case Point(x=1, x=2):` gives the attribute `x` two keyword sub-patterns. Python refuses this when the module is compiled. Only keyword sub-patterns are compared: matching a positional one to its attribute needs the class's `__match_args__`, which this rule does not read.

**Why:** The module does not import; the error shows up only when it runs.

```codelaw
match:
  kind: class_pattern
  has:
    kind: case_pattern
    has:
      kind: keyword_pattern
      has: {nthChild: 1, kind: identifier, pattern: $K}
    precedes:
      stopBy: end
      kind: case_pattern
      has:
        kind: keyword_pattern
        has: {nthChild: 1, kind: identifier, pattern: $K}
message: "`$K` is given more than one sub-pattern"
```

## How to fix

1. Keep the sub-pattern the case is meant to test, and remove the other.

## Examples

### Python

#### Don't

```python
match point:
    case Point(x=0, x=1):
        pass
```

#### Do

```python
match point:
    case Point(x=0, y=1):
        pass
```
