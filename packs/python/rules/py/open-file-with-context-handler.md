---
id: py/open-file-with-context-handler
title: "Open files in a with statement"
languages: [python]
fix: manual
fix_reason: "the code that uses the file moves into the `with` block"
fix_exception: restructure
stability: preview
ported_from: [ruff/SIM115]
since: 0.5.0
---

A file opened with `open()` and assigned to a name is opened in a `with` statement instead.

**Why:** `with` closes the file even when an exception is raised; a bare `open` leaks it until garbage collection.

```codelaw
match:
  kind: assignment
  has: {field: right, kind: call, has: {field: function, regex: '^(open|io\.open|codecs\.open|tempfile\.(TemporaryFile|NamedTemporaryFile))$'}}
  not: {inside: {kind: with_statement, stopBy: end}}
message: "Open the file in a `with` statement"
```

## How to fix

1. Write `with open(...) as f:` and move the code that uses `f` into the block.

## Examples

### Python

#### Don't

```python
f = open("data.txt")
data = f.read()
```

#### Do

```python
with open("data.txt") as f:
    data = f.read()
```
