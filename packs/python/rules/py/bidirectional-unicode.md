---
id: py/bidirectional-unicode
title: "No bidirectional control characters"
languages: [python]
fix: manual
fix_reason: "removing the character could hide what the code was made to look like; a person must read it"
fix_exception: security
stability: preview
ported_from: [ruff/PLE2502]
since: 0.5.0
---

Strings and comments contain no Unicode bidirectional control characters (U+202A–U+202E, U+2066–U+2069).

**Why:** They make code display differently from how it runs (the Trojan Source attack).

```codelaw
match:
  any: [{kind: string_content}, {kind: comment}]
  regex: '[\x{202A}-\x{202E}\x{2066}-\x{2069}]'
message: "This contains a bidirectional control character"
```

## How to fix

1. Remove the character, and check the code does what it looks like it does.

## Examples

### Python

#### Don't

```python
x = "abc‮def"
```

#### Do

```python
x = "abcdef"
```
