---
id: py/redundant-backslash
title: "No backslash inside brackets"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/E502]
since: 0.5.0
---

A line continuation `\` inside parentheses, brackets or braces is removed.

**Why:** Lines already continue inside brackets; the backslash is noise and breaks if a space follows it.

```codelaw
match:
  kind: line_continuation
  inside: {stopBy: end, any: [{kind: parenthesized_expression}, {kind: argument_list}, {kind: list}, {kind: tuple}, {kind: dictionary}, {kind: set}, {kind: parameters}, {kind: subscript}]}
message: "Remove the backslash; the brackets already continue the line"
fix: "\n"
```

## Examples

### Python

#### Don't

```python
total = (first + \
         second)
```

#### Do

```python
total = (first + 
         second)
```
