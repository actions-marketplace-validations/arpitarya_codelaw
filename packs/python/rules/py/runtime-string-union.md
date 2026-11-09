---
id: py/runtime-string-union
title: "A string is not a member of an `X | Y` union that runs"
languages: [python]
fix: manual
fix_reason: "either the quotes go from the member or they go around the whole union, and which one fits is the author's"
fix_exception: restructure
stability: preview
ported_from: [ruff/TC010]
since: 0.5.0
---

A string (a forward reference) joined to another type with `|` in an annotation that runs — a parameter or return annotation, or a module or class variable's, in a file without `from __future__ import annotations` — is reported: `"Foo" | None` raises a `TypeError` when the annotation is evaluated. A variable's annotation inside a function never runs and is not read.

**Why:** `str | None` is not defined, and a forward reference in a union only works when the whole union is the string.

```codelaw
native: py/runtime-string-union
```

## How to fix

1. Remove the quotes, wrap the whole union in quotes, or add `from __future__ import annotations`.

## Examples

### Python

#### Don't

```python
var: "Foo" | None = None


class Foo: ...
```

#### Do

```python
var: "Foo | None" = None


class Foo: ...
```
