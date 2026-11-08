---
id: py/reuse-of-groupby-generator
title: "A `groupby` group is used once"
languages: [python]
fix: suggest
fix_reason: "`group = list(group)` goes first in the loop body: the later uses see the items instead of an exhausted iterator, which is the fix and a change in what they get"
stability: preview
ported_from: [ruff/B031]
since: 0.1.0
---

The group iterator from `itertools.groupby()` is consumed once inside its loop.

**Why:** Each group is a one-shot iterator; a second pass, or a pass inside a nested loop, sees nothing.

```codelaw
native: py/reuse-of-groupby-generator
```

## How to fix

1. Materialise the group once with `list(group)` and reuse the list.

## Examples

### Python

#### Don't

```py
import itertools

for key, group in itertools.groupby(rows):
    first = list(group)
    rest = list(group)
```

#### Fixed

```py
import itertools

for key, group in itertools.groupby(rows):
    group = list(group)
    first = list(group)
    rest = list(group)
```

#### Do

```py
import itertools

for key, group in itertools.groupby(rows):
    items = list(group)
    first, rest = items[:1], items[1:]
```
