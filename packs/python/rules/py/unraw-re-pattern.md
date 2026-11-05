---
id: py/unraw-re-pattern
title: "Regex patterns are raw strings"
languages: [python]
fix: suggest
fix_reason: "adding `r` changes the string when it holds an escape Python already processed"
stability: preview
ported_from: [ruff/RUF039]
since: 0.5.0
---

A string literal passed as the pattern of a `re` function is a raw string.

**Why:** Without `r`, `\b` is a backspace and `\d` a deprecated escape; the regex silently means something else.

```codelaw
match:
  kind: string
  regex: '^["'']'
  has: {kind: string_content, regex: '\\'}
  nthChild: 1
  inside:
    kind: argument_list
    inside: {kind: call, has: {field: function, regex: '^re\.(compile|search|match|fullmatch|split|findall|finditer|sub|subn)$'}}
message: "Write the pattern as a raw string"
fix: "r$$MATCH"
```

## Examples

### Python

#### Don't

```python
found = re.search("\d+", text)
```

#### Do

```python
found = re.search(r"\d+", text)
```
