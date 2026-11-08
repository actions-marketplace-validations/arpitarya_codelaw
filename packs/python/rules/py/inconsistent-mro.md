---
id: py/inconsistent-mro
title: "A class's bases have a consistent method resolution order"
languages: [python]
fix: manual
fix_reason: "which base order the author meant is theirs"
fix_exception: restructure
stability: preview
ported_from: [pylint/inconsistent-mro]
since: 0.5.0
---

A class lists bases whose C3 linearization has no answer — typically a base before a subclass of it (`class C(A, B)` where `B` derives from `A`). Read from the classes of this file: a base the file imports makes the ancestry open, and the rule then says nothing; a base in another file is not followed (the project graph's class facts would answer it).

**Why:** Python raises `TypeError: Cannot create a consistent method resolution order (MRO)` when the class statement runs.

```codelaw
native: py/inconsistent-mro
```

## How to fix

1. List the more derived base first (`class C(B, A)`), or
2. drop the base the other one already brings.

## Examples

### Python

#### Don't

```python
class A: ...


class B(A): ...


class C(A, B): ...
```

#### Do

```python
class A: ...


class B(A): ...


class C(B, A): ...
```
