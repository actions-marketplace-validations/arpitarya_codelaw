---
id: py/format-literals
title: "`.format` fields use implicit positions"
languages: [python]
fix: auto
fix_reason: "auto while the arguments stay where they are; when the fields are out of order and a moved argument has a call in it, the arguments change their evaluation order and the finding is a suggestion"
stability: preview
ported_from: [ruff/UP030]
since: 0.5.0
---

A `"…".format(…)` string whose fields are all explicit positions (`{0}`, `{1}`) numbered from 0 with none missing drops the numbers: `"{} {}".format(a, b)`. Fields out of order (`{1} {0}`) take the arguments in the new order; strings with a keyword or an automatic field, a field that repeats a position, and nested fields are left alone.

**Why:** Implicit positions are what most `.format` calls use, and an explicit number is one more thing to keep in step with the arguments.

```codelaw
native: py/format-literals
```

## Examples

### Python

#### Don't

```python
message = "{0} is {1}".format(name, age)
```

#### Do

```python
message = "{} is {}".format(name, age)
```
