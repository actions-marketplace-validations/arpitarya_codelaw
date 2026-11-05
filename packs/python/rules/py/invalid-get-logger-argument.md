---
id: py/invalid-get-logger-argument
title: "getLogger(__name__), not getLogger(__file__)"
languages: [python]
fix: suggest
fix_reason: "the logger's name changes, which configured handlers match on"
stability: preview
ported_from: [ruff/LOG002]
since: 0.5.0
---

`logging.getLogger(__file__)` and `getLogger(__cached__)` are written `getLogger(__name__)`.

**Why:** Logger names are dotted module names; a file path does not fit the hierarchy.

```codelaw
match:
  kind: identifier
  regex: '^(__file__|__cached__)$'
  inside: {kind: argument_list, inside: {kind: call, has: {field: function, regex: '^logging\.getLogger$'}}}
message: "Use `__name__`"
fix: "__name__"
```

## Examples

### Python

#### Don't

```python
log = logging.getLogger(__file__)
```

#### Do

```python
log = logging.getLogger(__name__)
```
