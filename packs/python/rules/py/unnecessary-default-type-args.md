---
id: py/unnecessary-default-type-args
title: "Generator[int], not Generator[int, None, None]"
languages: [python]
fix: suggest
fix_reason: "omitting the defaults needs Python 3.13"
stability: preview
ported_from: [ruff/UP043]
since: 0.5.0
---

`Generator[Y, None, None]` is written `Generator[Y]`, and `AsyncGenerator[Y, None]` `AsyncGenerator[Y]` (Python 3.13+).

**Why:** `None` is the default for the send and return types.

```codelaw
match:
  kind: generic_type
  regex: ^((typing|collections\.abc)\.)?Generator\[
  all:
  - has:
      kind: type_parameter
      regex: ^\[[^\[\],]+,\s*None\s*,\s*None\s*\]$
      has: {kind: type, nthChild: 1, pattern: $Y}
  - has: {kind: identifier, pattern: $G}
message: Write `$G[$Y]`
fix: $G[$Y]
```

## Examples

### Python

#### Don't

```python
def gen() -> Generator[int, None, None]:
    yield 1
```

#### Do

```python
def gen() -> Generator[int]:
    yield 1
```
