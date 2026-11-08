---
id: py/multiple-starred-expressions
title: "One starred target per assignment"
languages: [python]
fix: suggest
fix_reason: "one choice per starred target: it keeps its star and the others lose theirs; a SyntaxError today, and which target collects the rest is the author's call, so none is the default"
stability: preview
ported_from: [ruff/F622, pylint/star-needs-assignment-target]
since: 0.1.0
---

An unpacking assignment has at most one starred target.

**Why:** Two starred targets are ambiguous and a `SyntaxError`.

```codelaw
native: py/multiple-starred-expressions
```

## How to fix

1. Keep the star on one target only.

## Examples

### Python

#### Don't

```py
*head, *tail = values
```

#### Fixed (keep *head)

```py
*head, tail = values
```

#### Fixed (keep *tail)

```py
head, *tail = values
```

#### Do

```py
head, *tail = values
```
