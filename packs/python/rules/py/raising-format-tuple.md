---
id: py/raising-format-tuple
title: "An exception message is formatted before raising"
languages: [python]
fix: manual
fix_reason: "the message's form is the author's"
fix_exception: value
stability: preview
ported_from: [pylint/raising-format-tuple]
since: 0.5.0
---

An exception is not raised with a `%` format string and its values as separate arguments: `raise ValueError("bad %s", x)`.

**Why:** Exceptions do not format their arguments; the message shows the raw tuple.

```codelaw
match:
  kind: call
  has: {field: arguments, all: [{has: {nthChild: 1, kind: string, regex: '%[-#0 +]*\d*(\.\d+)?[sdirfgx]'}}, {has: {nthChild: 2, not: {kind: keyword_argument}}}]}
  inside: {kind: raise_statement}
message: "Format the message before passing it"
```

## How to fix

1. Format the string first (an f-string) and pass the one message.

## Examples

### Python

#### Don't

```python
raise ValueError("bad %s", name)
```

#### Do

```python
raise ValueError(f"bad {name}")
```
