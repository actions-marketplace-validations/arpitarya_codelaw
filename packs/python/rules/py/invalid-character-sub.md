---
id: py/invalid-character-sub
title: "A SUB (control-Z) character in a string is written \\x1a"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/PLE2512]
since: 0.5.0
---

A literal SUB (control-Z) character (U+001A) inside a string or an f-string's text is written as its escape, `\x1a`. In a raw string, or right after a backslash, the character is reported and left for you, since the escape would change the value there.

**Why:** The character is invisible in an editor and a diff, and some tools stop at it; the escape shows what the string holds.

```codelaw
native: py/invalid-character-sub
```

## Examples

### Python

#### Don't

```python
SEP = "ab"
```

#### Do

```python
SEP = "a\x1ab"
```
