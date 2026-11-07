---
id: py/f-string-in-get-text-func-call
title: "No f-string passed to gettext"
languages: [python]
fix: manual
fix_reason: "the message template and its placeholders' names are the author's"
fix_exception: value
stability: preview
ported_from: [ruff/INT001]
since: 0.5.0
---

A translation call — `_()`, `gettext()`, `ngettext()` and their lazy and context variants — is not given an f-string: the text is looked up after it is filled in, so no translation matches.

**Why:** Translators translate the template; an f-string hands the catalogue a different string every time.

```codelaw
match:
  kind: string
  has: {kind: interpolation}
  inside: {kind: argument_list, inside: {kind: call, has: {field: function, regex: '^(_|gettext|ngettext|ugettext|ungettext|gettext_lazy|ngettext_lazy|pgettext|npgettext)$'}}}
message: "Translate the template, then fill it in"
```

## How to fix

1. Write the message with named placeholders, `_("Hello, {name}")`, and fill them after translating: `.format(name=name)`.

## Examples

### Python

#### Don't

```python
msg = _(f"Hello, {name}")
```

#### Do

```python
msg = _("Hello, {name}").format(name=name)
```
