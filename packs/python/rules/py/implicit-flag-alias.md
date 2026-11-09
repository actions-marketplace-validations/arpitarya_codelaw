---
id: py/implicit-flag-alias
title: "A multi-bit `Flag` member is written as a union"
languages: [python]
fix: manual
fix_reason: "which flags the number was meant to combine is the author's to confirm"
fix_exception: value
stability: preview
ported_from: [pylint/implicit-flag-alias]
since: 0.5.0
---

In a class deriving `enum.Flag` or `enum.IntFlag`, a member given as an integer literal that is not a power of two and shares bits with the single-bit members (or with an earlier multi-bit member that shares none): `C = 3` beside `A = 1` and `B = 2`. Members equal to another member's value, `auto()` members, zero, and members written as a union (`RW = R | W`) are not read.

**Why:** `C = 3` is an alias of `A | B` that reads like a new flag, and `D = 5` beside `A = 1` is one whose meaning depends on the other members' bits.

```codelaw
native: py/implicit-flag-alias
```

## How to fix

1. Write the union: `C = A | B`, or
2. give the member a bit no other member uses.

## Examples

### Python

#### Don't

```python
from enum import IntFlag


class Perm(IntFlag):
    R = 4
    W = 2
    X = 1
    RW = 6
```

#### Do

```python
from enum import IntFlag


class Perm(IntFlag):
    R = 4
    W = 2
    X = 1
    RW = R | W
```
