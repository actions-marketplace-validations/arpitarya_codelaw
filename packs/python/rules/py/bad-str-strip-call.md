---
id: py/bad-str-strip-call
title: "strip() takes a set of characters, not a word"
languages: [python]
fix: suggest
fix_reason: "the argument without its repeated characters strips exactly the same characters; if a prefix or suffix was meant, `removeprefix` or `removesuffix` is the fix, which only you know"
stability: preview
ported_from: [ruff/PLE1310, pylint/bad-str-strip-call]
since: 0.5.0
---

A `strip`, `lstrip` or `rstrip` call on a string (or bytes) whose argument literal repeats a character — `name.strip("www")` — is reported: the argument is a set of characters, so the repeat is a sign a prefix or suffix was meant. The receiver is a string literal, or a name every binding of which in its function is a literal or annotated `str` (`bytes` for a bytes argument); Ruff also follows other bindings through its semantic model. The suggestion drops the repeats, which strips the same characters; for `lstrip`/`rstrip` the message names `removeprefix`/`removesuffix` (Python 3.9+).

**Why:** `"wwwxyz.www".lstrip("www.")` strips every leading `w` and `.`, which is rarely what was meant.

```codelaw
native: py/bad-str-strip-call
```

## Examples

### Python

#### Don't

```python
tag = "--name--".strip("---")
```

#### Do

```python
tag = "--name--".strip("-")
```
