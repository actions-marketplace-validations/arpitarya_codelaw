---
id: py/bad-format-string-key
title: "A `%` format's dict keys are strings"
languages: [python]
fix: manual
fix_reason: "which name the key should have is in the format string, and only the author knows which field it was meant for"
fix_exception: value
stability: preview
ported_from: [pylint/bad-format-string-key]
since: 0.5.0
---

`"%(name)s" % {...}` looks fields up by string key, so a dict literal on the right with a key that is a number, `True`, `False` or `None` can never fill a field. A key that is not a constant is not read.

**Why:** The `%` operator raises `KeyError` for the field the key was meant to fill.

```codelaw
match:
  kind: binary_operator
  all:
    - has: {field: left, kind: string, regex: '%\('}
    - has: {field: operator, regex: '^%$'}
    - has:
        field: right
        kind: dictionary
        has:
          kind: pair
          has:
            field: key
            any: [{kind: integer}, {kind: float}, {kind: "true"}, {kind: "false"}, {kind: none}]
message: "A `%(name)` format looks keys up by string"
```

## How to fix

1. Use the field's name, as a string, for the key.

## Examples

### Python

#### Don't

```python
text = "%(count)d items" % {1: n}
```

#### Do

```python
text = "%(count)d items" % {"count": n}
```
