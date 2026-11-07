---
id: py/byte-string-usage
title: "No typing.ByteString"
languages: [python]
fix: manual
fix_reason: "which byte types the code accepts is the author's"
fix_exception: value
stability: preview
ported_from: [ruff/PYI057]
since: 0.5.0
---

`typing.ByteString` and `collections.abc.ByteString` are not used: they are deprecated and gone in Python 3.14.

**Why:** `ByteString` never matched what its name says (`memoryview` was not one), and it is removed.

```codelaw
match:
  any:
    - kind: attribute
      regex: '^(typing|collections\.abc)\.ByteString$'
    - kind: dotted_name
      regex: '^ByteString$'
      inside: {kind: import_from_statement, field: name, has: {field: module_name, regex: '^(typing|collections\.abc)$'}}
message: "`ByteString` is deprecated; name the types"
```

## How to fix

1. Annotate with the types the code accepts, such as `bytes | bytearray | memoryview`, or `collections.abc.Buffer`.

## Examples

### Python

#### Don't

```python
def f(b: typing.ByteString) -> None: ...
```

#### Do

```python
def f(b: bytes | bytearray) -> None: ...
```
