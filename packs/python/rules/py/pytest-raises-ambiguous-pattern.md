---
id: py/pytest-raises-ambiguous-pattern
title: "pytest.raises(match=) escapes regex characters"
languages: [python]
fix: manual
fix_reason: "whether the pattern was meant as a regex or literal text is the author's"
fix_pending: "suggest-choices; needs add-import (W-162)"
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
    - has: {field: value, kind: string, regex: '^["'']', not: {regex: '\\'}, has: {kind: string_content, regex: '[.*+?^$()\[\]{}|]'}}
    - inside: {kind: argument_list, inside: {kind: call, has: {field: function, regex: '^pytest\.raises$'}}}
message: "`match` is a regex; escape the metacharacters"
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
