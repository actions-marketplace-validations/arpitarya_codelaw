---
id: py/invalid-character-esc
title: "A ESC character in a string is written \\x1b"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/PLE2513]
since: 0.5.0
---

A literal ESC character (U+001B) inside a string or an f-string's text is written as its escape, `\x1b`. In a raw string, or right after a backslash, the character is reported and left for you, since the escape would change the value there.

**Why:** The character is invisible in an editor and a diff, and some tools stop at it; the escape shows what the string holds.

```codelaw
native: py/invalid-character-esc
```

## Examples

### Python

#### Don't

```python
SEP = "ab"
```

#### Do

```python
SEP = "a\x1bb"
```
