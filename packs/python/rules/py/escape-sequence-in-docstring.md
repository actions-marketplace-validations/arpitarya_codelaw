---
id: py/escape-sequence-in-docstring
title: "A docstring with a backslash is raw"
languages: [python]
fix: suggest
fix_reason: "an `r` prefix keeps every backslash as written; an escape like `\\n` that was meant becomes two characters"
stability: preview
ported_from: [ruff/D301]
since: 0.5.0
---

A docstring that holds a backslash, other than one ending a line, is written as a raw string, `r"""…"""`.

**Why:** In a plain string `\d` is an invalid escape, a warning today and an error later, and `\n` silently becomes a newline.

```codelaw
utils:
  docstring:
    kind: string
    inside:
      kind: expression_statement
      nthChild: 1
      any:
        - inside: {kind: block, inside: {kind: function_definition}}
        - inside: {kind: block, inside: {kind: class_definition}}
        - inside: {kind: module}
match:
  matches: docstring
  regex: '^(?:"""|'''''')[\s\S]*\\[^\n]'
message: "Make the docstring raw: `r\"\"\"…\"\"\"`"
fix:
  - insert-before: {target: match, text: r}
```

## Examples

### Python

#### Don't

```python
def digits(s):
    """Match \d+ in s."""
```

#### Do

```python
def digits(s):
    r"""Match \d+ in s."""
```
