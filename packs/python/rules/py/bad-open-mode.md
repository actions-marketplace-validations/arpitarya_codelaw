---
id: py/bad-open-mode
title: "open() takes a valid mode"
languages: [python]
fix: manual
fix_reason: "which mode was meant is the author's"
fix_exception: value
stability: preview
ported_from: [ruff/PLW1501]
since: 0.5.0
---

The mode given to `open()` is a valid one: one of `r`, `w`, `x`, `a`, optionally with `+`, `b` or `t`.

**Why:** An invalid mode raises ValueError when the line runs.

```codelaw
match:
  kind: string
  has: {kind: string_content, not: {regex: '^(?:[rwxa]\+?[bt]?|[rwxa][bt]\+|[bt][rwxa]\+?|[rwxa]\+[bt]|U|rU|Ur|rbU|rUb|Ubr|Urb)$'}}
  not: {regex: '^[fF]'}
  any:
    - inside: {kind: argument_list, has: {nthChild: 2, kind: string, pattern: $$MATCH}, inside: {kind: call, has: {field: function, regex: '^(open|io\.open)$'}}}
    - inside: {kind: keyword_argument, has: {field: name, regex: '^mode$'}, inside: {kind: argument_list, inside: {kind: call, has: {field: function, regex: '^(open|io\.open)$'}}}}
message: "This is not a valid file mode"
```

## How to fix

1. Use a valid mode such as `"r"`, `"w"`, `"rb"` or `"a+"`.

## Examples

### Python

#### Don't

```python
f = open(path, "rw")
```

#### Do

```python
f = open(path, "r+")
```
