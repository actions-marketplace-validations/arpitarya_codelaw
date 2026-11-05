---
id: py/unnecessary-lambda
title: "No lambda that only calls a function"
languages: [python]
fix: suggest
fix_reason: "the function is looked up now instead of at each call"
stability: preview
ported_from: [ruff/PLW0108]
since: 0.5.0
---

`lambda x: f(x)` is written `f`.

**Why:** The lambda adds a call and a name for nothing.

```codelaw
match:
  kind: lambda
  pattern: "lambda $A: $F($A)"
constraints:
  A: {kind: identifier}
  F: {any: [{kind: identifier}, {kind: attribute}], not: {has: {stopBy: end, kind: call}}}
message: "Pass `$F` directly"
fix: "$F"
```

## Examples

### Python

#### Don't

```python
names = map(lambda n: str(n), nums)
```

#### Do

```python
names = map(str, nums)
```
