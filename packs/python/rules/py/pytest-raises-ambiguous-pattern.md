---
id: py/pytest-raises-ambiguous-pattern
title: "pytest.raises(match=) escapes regex characters"
languages: [python]
fix: suggest
fix_reason: "the fix reads the pattern as literal text and escapes it in a raw string; a pattern meant as a regular expression is the author's to keep"
stability: preview
ported_from: [ruff/RUF043]
since: 0.5.0
---

The `match=` of `pytest.raises` given a plain (non-raw) string holding regex metacharacters and no escapes is reported.

**Why:** `match` is a regular expression; `"1.5"` also matches `"105"`, so the test passes for the wrong message.

```codelaw
match:
  kind: keyword_argument
  all:
    - has: {field: name, regex: '^match$'}
    - has: {field: value, kind: string, regex: '^["'']', not: {regex: '\\'}, has: {kind: string_content, regex: '[.*+?^$()\[\]{}|]'}, pattern: $S}
    - inside: {kind: argument_list, inside: {kind: call, has: {field: function, regex: '^pytest\.raises$'}}}
message: "`match` is a regex; escape the metacharacters"
fix:
  - edit-text: { target: $S, pattern: '([.*+?^$()\[\]{}|])', replace: '\${1}' }
  - edit-text: { target: $S, pattern: '\A', replace: 'r' }
```

## How to fix

1. Escape the metacharacters (`r"1\.5"`) or wrap the text in `re.escape(...)`.

## Examples

### Python

#### Don't

```python
with pytest.raises(ValueError, match="1.5"):
    parse()
```

#### Do

```python
with pytest.raises(ValueError, match=r"1\.5"):
    parse()
```
