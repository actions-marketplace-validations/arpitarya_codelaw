---
id: py/invalid-character-zero-width-space
title: "A zero-width space character in a string is written \\u200b"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/PLE2515]
since: 0.5.0
---

A literal zero-width space character (U+200B) inside a string or an f-string's text is written as its escape, `\u200b`. In a raw string, or right after a backslash, the character is reported and left for you, since the escape would change the value there.

**Why:** The character is invisible in an editor and a diff, and some tools stop at it; the escape shows what the string holds.

```codelaw
native: py/invalid-character-zero-width-space
```

## Examples

### Python

#### Don't

```python
SEP = "a​b"
```

#### Do

```python
SEP = "a\u200bb"
```
