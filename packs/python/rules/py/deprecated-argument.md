---
id: py/deprecated-argument
title: "No argument the documentation deprecates"
languages: [python]
fix: manual
fix_reason: "whether to drop the argument or pass something else in its place depends on what it did, which the table does not hold"
fix_exception: value
finding_target: { python: { from: "3.0" } }
stability: preview
ported_from: [pylint/deprecated-argument]
since: 0.5.0
---

A call that passes an argument CPython's documentation deprecates by name (`xml.etree.ElementTree.iterparse(…, parser=…)`, `select.epoll(flags=…)`, `email.utils.localtime(isdst=…)`), by keyword, or by position when the table knows it and no `*args` makes the position uncertain; reported when the project's target is at or past the version that deprecates it. Only a parameter the documentation deprecates by itself is in the table: `Passing x positionally is deprecated` is not.

**The table** is the data package `@codelaw/data-python-deprecations` / `codelaw-data-python-deprecations` (generated from CPython's library reference, the Python Software Foundation License; L6 decision 15), read only when this rule is on. Without the package the rule reports nothing. It lists only names the documentation deprecates as a whole, by the Python version that deprecates them (and the one that removes them): a deprecation of one way of calling a function is not in it.

**The target** is the project's oldest supported Python (`[targets] python`, else `requires-python`; SR-CONFIG decision 11f). A name is reported once the target has reached the version that deprecates it, as Pylint's `py-version` does; with no target the rule does nothing.

**Why:** A deprecated argument is ignored or warns, and is removed in a later Python: the call then raises `TypeError`.

```codelaw
native: py/deprecated-argument
```

## How to fix

1. Read the documentation of the function: the deprecation note says what the argument did and what replaces it.
2. Drop the argument, or pass its replacement.

## Examples

### Python

#### Don't

```python python=3.9
import xml.etree.ElementTree as ET

for event, element in ET.iterparse(path, events=("end",), parser=parser):
    pass
```

#### Do

```python python=3.9
import xml.etree.ElementTree as ET

for event, element in ET.iterparse(path, events=("end",)):
    pass
```

## Differences from the source

Pylint's table of deprecated arguments is its own and larger; codelaw's holds the few arguments the documentation deprecates by themselves. A function whose deprecation depends on how it is called (`asyncio.ensure_future` without a loop) is not reported.
