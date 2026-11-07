---
id: py/consider-using-with
title: "A resource is opened with with"
languages: [python]
fix: manual
fix_reason: "the code that uses the resource moves into the `with` block"
fix_exception: restructure
stability: preview
ported_from: [pylint/consider-using-with]
since: 0.5.0
---

`open(...)`, `subprocess.Popen(...)` and `tempfile.TemporaryFile(...)`-style calls assigned to a name are written as a `with` statement instead.

**Why:** `with` closes the resource even when an exception is raised; a plain assignment leaks it on every error path.

```codelaw
match:
  kind: assignment
  not: {inside: {kind: function_definition, has: {field: name, regex: '^__(init|enter)__$'}, stopBy: end}}
  all:
    - has: {field: left, kind: identifier}
    - has: {field: right, kind: call, has: {field: function, regex: '^(open|io\.open|codecs\.open|subprocess\.Popen|tempfile\.(TemporaryFile|NamedTemporaryFile|SpooledTemporaryFile|TemporaryDirectory)|zipfile\.ZipFile|tarfile\.open)$'}}
message: "Open it in a `with` statement"
```

## How to fix

1. Write `with open(...) as f:` and move the code that uses it into the block.

## Examples

### Python

#### Don't

```python
f = open(path)
data = f.read()
f.close()
```

#### Do

```python
with open(path) as f:
    data = f.read()
```
