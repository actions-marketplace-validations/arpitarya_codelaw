---
id: py/unnecessary-nested-literal
title: "Literal[a, b], not Literal[a, Literal[b]]"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/RUF041]
since: 0.5.0
---

A `Literal[...]` is not nested inside another `Literal[...]`.

**Why:** Nested literals flatten to the same type; the nesting is noise.

```codelaw
match:
  kind: subscript
  regex: '^(typing\.)?Literal\['
  has: {kind: subscript, regex: '^(typing\.)?Literal\[', pattern: $N, has: {field: subscript, pattern: $V}}
  not: {inside: {kind: subscript, regex: '^(typing\.)?Literal\['}}
message: "Flatten the nested `Literal`"
fix:
  - replace: {target: $N, with: "$V"}
```

## Examples

### Python

#### Don't

```python
Mode = Literal["r", Literal["w"]]
```

#### Do

```python
Mode = Literal["r", "w"]
```
