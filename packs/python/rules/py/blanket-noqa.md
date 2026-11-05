---
id: py/blanket-noqa
title: "noqa names its rule codes"
languages: [python]
fix: manual
fix_reason: "the codes to name are the linter's, not in the source"
fix_exception: value
stability: preview
ported_from: [ruff/PGH004]
since: 0.5.0
---

A `# noqa` comment names the rules it silences: `# noqa: E501`.

**Why:** A blanket `noqa` also hides every new finding on the line.

```codelaw
match:
  kind: comment
  regex: '(?i)#\s*noqa\s*($|#)|(?i)#\s*noqa:\s*$'
message: "Name the rule codes: `# noqa: CODE`"
```

## How to fix

1. Write the codes the linter reports on the line: `# noqa: E501`.

## Examples

### Python

#### Don't

```python
import os  # noqa
```

#### Do

```python
import os  # noqa: F401
```
