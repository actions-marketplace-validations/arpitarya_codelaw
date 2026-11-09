---
id: py/f-string
title: "A `.format` call on a string literal is an f-string"
languages: [python]
fix: auto
fix_reason: "auto when each argument is evaluated once and in the order it was written; when a call is evaluated in another order, the finding is a suggestion; a call with a comment inside it is reported without an edit"
stability: preview
ported_from: [ruff/UP032, pylint/consider-using-f-string]
since: 0.5.0
---

`"{} and {}".format(a, b)` is `f"{a} and {b}"`: each field takes its argument's source text, an attribute or index of the field (`{0.name}`, `{0[key]}`) follows it, and the conversion and spec stay. Not rewritten: arguments with `*` or `**`, with a quote or a line break in their text, arguments the string never uses, an argument with a call used twice, a spec holding another field, an implicit string concatenation, and a call inside Django's `gettext`. An argument that needs parentheses inside braces (a lambda, a dict or set display, a walrus, a bare generator) gets them.

**Why:** An f-string puts the value where it is printed; `.format` makes the reader match fields to arguments.

```codelaw
native: py/f-string
```

## Examples

### Python

#### Don't

```python
line = "{} and {}".format(first, second)
```

#### Do

```python
line = f"{first} and {second}"
```
