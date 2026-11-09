---
id: py/split-static-string
title: "A static `str.split` is a list literal"
languages: [python]
fix: auto
fix_reason: "auto unless a comment sits inside the call (it would be lost: that rewrite is a suggestion)"
stability: preview
ported_from: [ruff/SIM905]
since: 0.5.0
---

A string literal (or implicit concatenation of literals) split by a call to `split` or `rsplit` whose arguments are literals — a `sep` that is a string or `None` and a `maxsplit` that is an integer, positionally or by keyword — is written as the list it makes: `"a,b,c".split(",")` is `["a", "b", "c"]`. Without a `sep`, or with `None`, the string is split on runs of white space as Python does (the empty strings at the ends dropped; with a `maxsplit` the leading white space, or for `rsplit` the trailing, is dropped and the rest after the last split is kept whole). The elements are written in the quote style and prefix of the first part of the literal: a raw string keeps its quotes unless an element holds one, then the other quote, and triple quotes when it holds both. A call with a `sep` or `maxsplit` that is not a literal, an f-string or a bytes literal, an empty `sep` (a `ValueError`) and `splitlines` are not read.

**Why:** A list literal is plainer than a method call that always produces the same list.

```codelaw
native: py/split-static-string
```

## Examples

### Python

#### Don't

```python
CODES = "a,b,c,d".split(",")
```

#### Do

```python
CODES = ["a", "b", "c", "d"]
```
