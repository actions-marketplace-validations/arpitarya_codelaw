---
id: py/re-sub-positional-args
title: "`re.sub` count and flags by keyword"
languages: [python]
fix: auto
fix_reason: "the arguments after the third (`re.sub`, `re.subn`) or second (`re.split`) become `count=`/`maxsplit=` and `flags=`, by the real signature: the same binding. A flag in the `count` place is the bug the rule is for: its choices are `count=` (what runs today) and `flags=`"
stability: preview
ported_from: [ruff/B034]
since: 0.1.0
---

`re.sub()`, `re.subn()` and `re.split()` receive `count`, `maxsplit` and `flags` as keyword arguments.

**Why:** Their positions are easy to confuse: `re.sub(p, r, s, re.IGNORECASE)` passes the flag as `count`.

```codelaw
native: py/re-sub-positional-args
```

## How to fix

1. Pass `count=`, `maxsplit=` and `flags=` by name.

## Examples

### Python

#### Don't

```py
import re

text = re.sub("a", "b", text, 1)
```

#### Fixed

```py
import re

text = re.sub("a", "b", text, count=1)
```

#### Do

```py
import re

text = re.sub("a", "b", text, count=1)
```
