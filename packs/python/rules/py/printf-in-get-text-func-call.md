---
id: py/printf-in-get-text-func-call
title: "No % formatting inside a gettext call"
languages: [python]
fix: suggest
fix_reason: "`%` moves outside the call; a catalogue entry under the old filled-in text, if any, no longer matches"
stability: preview
ported_from: [ruff/INT003]
since: 0.5.0
---

`_("…" % args)` is written `_("…") % args`, so the template is translated, then filled in.

**Why:** Translators translate the template; formatting first hands the catalogue a different string every time.

```codelaw
match:
  kind: call
  all:
    - has: {field: function, pattern: $F, regex: '^(_|gettext|ngettext|ugettext|ungettext|gettext_lazy|ngettext_lazy|pgettext|npgettext)$'}
    - has: {field: arguments, all: [{has: {nthChild: 1, kind: binary_operator, pattern: '$S % $A'}}, {not: {has: {nthChild: 2}}}]}
constraints:
  S: {kind: string}
message: "Translate the template, then format it"
fix: "$F($S) % $A"
```

## Examples

### Python

#### Don't

```python
msg = _("Hello, %s" % name)
```

#### Do

```python
msg = _("Hello, %s") % name
```
