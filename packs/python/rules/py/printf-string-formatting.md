---
id: py/printf-string-formatting
title: "`%` formatting becomes `.format`"
languages: [python]
fix: suggest
fix_reason: "`\"%s\" % x` formats a tuple `x` as several values while `.format(x)` formats it as one, and `%` calls `str` where `.format` calls `format`; the rewrite is offered, not applied"
stability: preview
ported_from: [ruff/UP031]
since: 0.5.0
---

A string literal formatted with `%` (`"%s is %s" % (a, b)`) is written with `.format` fields and arguments: `%s` is `{}`, `%r` and `%a` are `{!r}` and `{!a}`, `%(name)s` is `{name}` with a keyword argument, width, precision and flags move into the field's spec (`%.2f` is `{:.2f}`). A right-hand side that is a name or call stands for one value, or for a tuple or mapping when the string has several or named conversions. Conversions with no equivalent (`%d`, `%i`, `%u`, `%c`, `*` widths, a width on `%s`) are reported without an edit.

**Why:** `.format` is clearer for long strings, and the f-string rewrite (`py/f-string`) starts from it.

```codelaw
native: py/printf-string-formatting
```

## Examples

### Python

#### Don't

```python
line = "%s is %s years old" % (name, age)
```

#### Do

```python
line = "{} is {} years old".format(name, age)
```
