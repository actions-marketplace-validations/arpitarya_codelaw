---
id: py/f-string-missing-placeholders
title: "An f-string has a placeholder"
languages: [python]
fix: auto
on_edit: false
fix_reason: "Deletes a placeholder still being filled in, so save and the agent hook leave it for codelaw fix."
stability: preview
ported_from: [ruff/F541, pylint/f-string-without-interpolation]
since: 0.1.0
---

A string with the `f` prefix contains at least one `{…}` placeholder.

**Why:** Without one, the prefix does nothing and often means a placeholder was forgotten.

```codelaw
native: py/f-string-missing-placeholders
```

## Examples

### Python

#### Don't

```py
print(f"hello")
```

#### Do

```py
print("hello")
```
