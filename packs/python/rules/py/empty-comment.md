---
id: py/empty-comment
title: "No empty comments"
languages: [python]
fix: auto
on_edit: false
fix_reason: "Deletes a placeholder still being filled in, so save and the agent hook leave it for codelaw fix."
stability: preview
ported_from: [ruff/PLR2044]
since: 0.5.0
---

A comment with no text after `#` is removed.

**Why:** It says nothing.

```codelaw
match:
  kind: comment
  regex: '^#\s*$'
message: "Remove the empty comment"
fix: [remove]
```

## Examples

### Python

#### Don't

```python
#
x = 1
```

#### Do

```python
x = 1
```
