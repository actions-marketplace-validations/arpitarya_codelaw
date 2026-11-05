---
id: py/unnecessary-literal-dict
title: "A dict literal, not dict([(k, v)])"
languages: [python]
fix: auto
fix_reason: "auto for one pair; longer lists are reported without a fix"
stability: preview
ported_from: [ruff/C406]
since: 0.5.0
---

`dict([(k, v)])` is written `{k: v}`.

**Why:** The literal builds the dict directly.

```codelaw
match:
  any:
    - pattern: dict([($K, $V)])
    - pattern: dict((($K, $V),))
message: "Write a dict literal"
fix: "{$K: $V}"
```

## Examples

### Python

#### Don't

```python
cfg = dict([("debug", True)])
```

#### Do

```python
cfg = {"debug": True}
```
