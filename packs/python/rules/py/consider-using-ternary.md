---
id: py/consider-using-ternary
title: "b if a else c, not a and b or c"
languages: [python]
fix: suggest
fix_reason: "the conditional also returns `b` when it is falsy, which `and`/`or` did not"
stability: preview
ported_from: [pylint/consider-using-ternary]
since: 0.5.0
---

`a and b or c` is written `b if a else c`.

**Why:** The `and`/`or` idiom returns `c` when `b` is falsy, which is rarely meant.

```codelaw
match:
  pattern: $A and $B or $C
  not: {inside: {kind: boolean_operator}}
constraints:
  A: {not: {kind: boolean_operator}}
message: "Write `$B if $A else $C`"
fix: "$B if $A else $C"
```

## Examples

### Python

#### Don't

```python
label = ok and "yes" or "no"
```

#### Do

```python
label = "yes" if ok else "no"
```
