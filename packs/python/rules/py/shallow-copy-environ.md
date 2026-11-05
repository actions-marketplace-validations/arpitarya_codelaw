---
id: py/shallow-copy-environ
title: "os.environ.copy(), not copy.copy(os.environ)"
languages: [python]
fix: suggest
fix_reason: "the copy becomes a plain dict that no longer writes through to the environment, which is what was meant"
stability: preview
ported_from: [ruff/PLW1507]
since: 0.5.0
---

`copy.copy(os.environ)` is written `os.environ.copy()`.

**Why:** `copy.copy` of `os.environ` shares the underlying environment, so changes to the copy change the process environment.

```codelaw
match:
  pattern: copy.copy(os.environ)
message: "Use `os.environ.copy()`"
fix: "os.environ.copy()"
```

## Examples

### Python

#### Don't

```python
env = copy.copy(os.environ)
```

#### Do

```python
env = os.environ.copy()
```
