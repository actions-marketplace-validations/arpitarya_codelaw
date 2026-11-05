---
id: py/format-unused-named-argument
title: "Every keyword passed to `.format` is used"
languages: [python]
fix: auto
on_edit: false
fix_reason: "Deletes a name a draft may be about to use, so save and the agent hook leave it for codelaw fix."
stability: preview
ported_from: [ruff/F522]
since: 0.1.0
---

A `.format()` call has no keyword arguments the string never names.

**Why:** An unused keyword is usually a misspelled field name.

```codelaw
native: py/format-unused-named-argument
```

## Examples

### Python

#### Don't

```py
message = "{name}".format(name="Ada", age=36)
```

#### Do

```py
message = "{name}".format(name="Ada")
```

## Differences from the source

The fix is offered only when the removed values are plain names or literals.
