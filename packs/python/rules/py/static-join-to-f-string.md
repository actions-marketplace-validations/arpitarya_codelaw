---
id: py/static-join-to-f-string
title: "A `join` of a fixed list is an f-string"
languages: [python]
fix: suggest
fix_reason: "`join` raises on an element that is not a string where an f-string formats it, so the rewrite is offered, not applied"
stability: preview
ported_from: [ruff/FLY002]
since: 0.5.0
---

`"-".join([a, b])` on a string literal and a list or tuple display of known length is the f-string `f"{a}-{b}"`; a list of string literals only becomes the joined string. A literal with a quote, backslash, brace or line break, a starred element, an element that is a string literal that is not plain, and an element with a quote in its text are left alone.

**Why:** The f-string shows the result's shape and costs no call.

```codelaw
native: py/static-join-to-f-string
```

## Examples

### Python

#### Don't

```python
name = ", ".join(["Dr", last])
```

#### Do

```python
name = f"Dr, {last}"
```
