---
id: py/raise-vanilla-args
title: "A long exception message lives in the exception class"
languages: [python]
fix: manual
fix_reason: "the exception class and the arguments it takes are the author's to design"
fix_exception: restructure
stability: preview
ported_from: [ruff/TRY003]
since: 0.5.0
---

A `raise` builds an exception with a message of several words as its first argument: `raise ParseError(f"line {n} is not valid")`. Write the message in the exception class and pass it only the values. A message is several words when its literal text holds whitespace; the values interpolated into an f-string do not count, and a one-word code (`"bad_input"`) is fine. `NotImplementedError` is left alone.

**Why:** The same message is retyped at each `raise` and drifts; a class that builds it keeps one wording and lets a handler read the values.

```codelaw
match:
  kind: call
  inside: {kind: raise_statement}
  all:
    - not: {has: {field: function, regex: '^(builtins\.)?NotImplementedError$'}}
    - has:
        field: arguments
        has:
          nthChild: 1
          any:
            - kind: string
            - kind: concatenated_string
          not: {regex: '^[rRuU]*[bBtT]'}
          has: {stopBy: end, kind: string_content, regex: '\s'}
message: "Move this message into the exception class"
```

## How to fix

1. Give the exception class an `__init__` that takes the values and builds the message.
2. Raise it with the values only.

## Examples

### Python

#### Don't

```python
raise ParseError(f"line {n} is not valid")
```

#### Do

```python
class ParseError(Exception):
    def __init__(self, line):
        super().__init__(f"line {line} is not valid")


raise ParseError(n)
```
