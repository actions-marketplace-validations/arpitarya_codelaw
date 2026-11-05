---
id: py/implicit-string-concatenation-in-collection-literal
title: "Parenthesize implicit concatenation in a list"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/ISC004]
since: 0.5.0
---

Two adjacent string literals inside a list, tuple or set literal are wrapped in parentheses.

**Why:** Unwrapped, they read like a missing comma — which they usually are.

```codelaw
match:
  kind: concatenated_string
  inside: {any: [{kind: list}, {kind: set}, {kind: tuple}]}
message: "Parenthesize this, or add the missing comma"
fix: "($$MATCH)"
```

## Examples

### Python

#### Don't

```python
names = ["a", "b" "c"]
```

#### Do

```python
names = ["a", ("b" "c")]
```
