---
id: py/docstring-tab-indentation
title: "A docstring is indented with spaces"
languages: [python]
fix: manual
fix_reason: "how many spaces a tab stood for is the author's"
fix_exception: value
stability: preview
ported_from: [ruff/D206]
since: 0.5.0
---

The lines of a docstring are indented with spaces, not tabs.

**Why:** `inspect.cleandoc` expands tabs to eight columns, so a tab-indented docstring renders out of line with the code.

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
  regex: '\n[ ]*\t'
message: "Indent the docstring with spaces"
```

## How to fix

1. Replace the tabs at the start of each docstring line with the spaces the code uses.

## Examples

### Python

#### Don't

```python
def f():
    """Do it.

	With a tab.
    """
```

#### Do

```python
def f():
    """Do it.

    With spaces.
    """
```
