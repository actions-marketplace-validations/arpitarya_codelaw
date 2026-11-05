---
id: py/duplicate-literal-member
title: "No value twice in a Literal"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/PYI062]
since: 0.5.0
---

A `Literal[...]` lists each value once.

**Why:** The duplicate adds nothing.

```codelaw
match:
  kind: type
  pattern: $V
  follows: {stopBy: end, kind: type, pattern: $V}
  inside: {kind: type_parameter, inside: {kind: generic_type, regex: '^(typing\.)?Literal\['}}
message: "This value is already listed"
fix: [remove]
```

## Examples

### Python

#### Don't

```python
def f(m: Literal["r", "w", "r"]) -> None: ...
```

#### Do

```python
def f(m: Literal["r", "w"]) -> None: ...
```
