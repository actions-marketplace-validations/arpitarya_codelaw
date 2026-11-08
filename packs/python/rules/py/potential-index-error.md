---
id: py/potential-index-error
title: "An index is inside a literal's length"
languages: [python]
fix: manual
fix_reason: "which element was meant is the author's"
fix_exception: value
stability: preview
ported_from: [ruff/PLE0643, pylint/potential-index-error]
since: 0.5.0
---

Indexing a tuple or list literal with an integer literal outside its length — `(1, 2, 3)[3]`, `[a, b][-3]` — is reported. A literal with a starred element is not read.

**Why:** The expression always raises `IndexError`.

```codelaw
native: py/potential-index-error
```

## How to fix

1. Use an index inside the sequence, or the element you meant.

## Examples

### Python

#### Don't

```python
color = ("red", "green")[2]
```

#### Do

```python
color = ("red", "green")[1]
```
