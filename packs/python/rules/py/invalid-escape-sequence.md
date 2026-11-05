---
id: py/invalid-escape-sequence
title: "No invalid escapes in strings"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/W605, pylint/anomalous-backslash-in-string]
since: 0.5.0
---

A non-raw string with an invalid escape such as `\d` is made raw when it has no valid escapes.

**Why:** Invalid escapes are a SyntaxWarning since Python 3.12 and will become an error.

```codelaw
match:
  kind: string
  regex: ^["']
  all:
  - has: {kind: string_content, regex: '\\[^\n\\''"abfnrtvx0-7NuU]'}
  - not:
      has: {kind: escape_sequence}
  - not:
      has: {kind: interpolation}
message: Make the string raw
fix: r$$MATCH
```

## Examples

### Python

#### Don't

```python
pat = "\d+"
```

#### Do

```python
pat = r"\d+"
```
