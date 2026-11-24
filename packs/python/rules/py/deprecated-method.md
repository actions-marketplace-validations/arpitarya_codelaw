---
id: py/deprecated-method
title: "No standard-library function or method the documentation deprecates"
languages: [python]
fix: manual
fix_reason: "the replacement is another function the documentation names, with another signature, which the table does not hold"
fix_exception: value
finding_target: { python: { from: "3.0" } }
stability: preview
ported_from: [pylint/deprecated-method]
since: 0.5.0
---

A call of a standard-library function or method that CPython's documentation deprecates (`datetime.datetime.utcnow()`, `inspect.getcallargs()`, `codecs.open()`, `ssl.SSLSocket.read`), reported when the project's target is at or past the version that deprecates it. The callee is read as the file's imports resolve it: `datetime.datetime.utcnow()` and `from datetime import datetime` then `datetime.utcnow()` are the same name; a method called on a variable whose class is not known is not reported.

**The table** is the data package `@codelaw/data-python-deprecations` / `codelaw-data-python-deprecations` (generated from CPython's library reference, the Python Software Foundation License; L6 decision 15), read only when this rule is on. Without the package the rule reports nothing. It lists only names the documentation deprecates as a whole, by the Python version that deprecates them (and the one that removes them): a deprecation of one way of calling a function is not in it.

**The target** is the project's oldest supported Python (`[targets] python`, else `requires-python`; SR-CONFIG decision 11f). A name is reported once the target has reached the version that deprecates it, as Pylint's `py-version` does; with no target the rule does nothing.

**Why:** A deprecated function warns when called and is removed in a later Python; some (`datetime.utcnow()`) also return a value that is easy to misuse.

```codelaw
native: py/deprecated-method
```

## How to fix

1. Read the documentation of the function: its deprecation note names the replacement (`datetime.now(datetime.timezone.utc)` for `datetime.utcnow()`).
2. Replace the call; where the replacement returns another type (an aware `datetime`), check what is done with the result.

## Examples

### Python

#### Don't

```python python=3.12
import datetime

now = datetime.datetime.utcnow()
```

#### Do

```python python=3.12
import datetime

now = datetime.datetime.now(datetime.timezone.utc)
```

## Differences from the source

Pylint's table of deprecated methods is its own and larger; codelaw's is the documentation's. The callee is read through the file's imports, and through a class built on the spot (`threading.Thread(…).getName()`); a method of a variable is not reported.
