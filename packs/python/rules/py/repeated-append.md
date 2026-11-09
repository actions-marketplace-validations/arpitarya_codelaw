---
id: py/repeated-append
title: "Consecutive appends to a list are one `extend`"
languages: [python]
fix: suggest
fix_reason: "`extend` takes every item at once: an item that reads the list itself, or an append that raises, behaves differently"
stability: preview
ported_from: [ruff/FURB113]
since: 0.5.0
---

Two or more `xs.append(a)` statements in a row on a name bound once to a list (a list display, a comprehension or `list(...)`) become `xs.extend((a, b))`. Appends to the same list that other statements separate are reported without an edit, and so are appends with a comment between them.

**Why:** One call says what the statements say together, and the list is grown once.

```codelaw
native: py/repeated-append
```

## Examples

### Python

#### Don't

```python
names = []
names.append("a")
names.append("b")
```

#### Do

```python
names = []
names.extend(("a", "b"))
```
