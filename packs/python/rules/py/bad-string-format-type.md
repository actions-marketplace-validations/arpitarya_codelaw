---
id: py/bad-string-format-type
title: "A `%` format's conversions match its literal arguments"
languages: [python]
fix: manual
fix_reason: "whether the format or the argument is the wrong one is the author's"
fix_exception: value
stability: preview
ported_from: [ruff/PLE1307]
since: 0.5.0
---

`'…' % args` where the format is a string literal and each argument is a literal of a type its conversion rejects: a string, bytes, container, `None` or `...` for `%d`, `%i`, `%x`, `%f` and the other number conversions; a float for `%x`, `%o`, `%b`, `%c`. `%s`, `%r` and `%a` take anything. The arguments may be a tuple (one per conversion), a dict with `%(key)d` conversions, or one value for a one-conversion format. A `*` width, an f-string format, and a non-literal argument are not read. The type comes from where this file states it — a literal, a `dict()`/`set()` call, a comprehension or an annotation — for a name bound once in its scope and never reassigned; a value from a call into other code is unknown and never reported.

**Why:** Python raises `TypeError: %d format: a real number is required, not str`.

```codelaw
native: py/bad-string-format-type
```

## How to fix

1. Use the conversion that matches the argument (`%s`), or
2. pass the number the format expects.

## Examples

### Python

#### Don't

```python
print('%d items' % 'three')
```

#### Do

```python
print('%s items' % 'three')
```
