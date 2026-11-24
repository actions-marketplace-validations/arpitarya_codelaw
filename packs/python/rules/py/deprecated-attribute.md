---
id: py/deprecated-attribute
title: "No standard-library attribute the documentation deprecates"
languages: [python]
fix: manual
fix_reason: "the replacement is another constant or attribute the documentation names, which the table does not hold, and its uses change with it"
fix_exception: value
finding_target: { python: { from: "3.0" } }
stability: preview
ported_from: [pylint/deprecated-attribute]
since: 0.5.0
---

A module attribute or constant that CPython's documentation deprecates, named in a `from module import NAME` or as `module.NAME` (`ssl.PROTOCOL_TLS`, `ssl.OP_NO_TLSv1`, `dis.hasjrel`, `decimal.HAVE_THREADS`), reported when the project's target is at or past the version that deprecates it. The name is read as the file's imports resolve it: an attribute of a variable whose type is not known is not reported.

**The table** is the data package `@codelaw/data-python-deprecations` / `codelaw-data-python-deprecations` (generated from CPython's library reference, the Python Software Foundation License; L6 decision 15), read only when this rule is on. Without the package the rule reports nothing. It lists only names the documentation deprecates as a whole, by the Python version that deprecates them (and the one that removes them): a deprecation of one way of calling a function is not in it.

**The target** is the project's oldest supported Python (`[targets] python`, else `requires-python`; SR-CONFIG decision 11f). A name is reported once the target has reached the version that deprecates it, as Pylint's `py-version` does; with no target the rule does nothing.

**Why:** A deprecated attribute warns when used, or quietly stops meaning what it did, and is removed in a later Python.

```codelaw
native: py/deprecated-attribute
```

## How to fix

1. Read the documentation of the attribute: its deprecation note names what to use (`ssl.PROTOCOL_TLS_CLIENT` or `PROTOCOL_TLS_SERVER` for `ssl.PROTOCOL_TLS`).
2. Replace it; if the attribute selects a behaviour, check the replacement's default.

## Examples

### Python

#### Don't

```python python=3.10
import ssl

context = ssl.SSLContext(ssl.PROTOCOL_TLS)
```

#### Do

```python python=3.10
import ssl

context = ssl.SSLContext(ssl.PROTOCOL_TLS_CLIENT)
```

## Differences from the source

Pylint's table of deprecated attributes is its own and larger; codelaw's is the documentation's, and it reads an attribute through the file's imports only, not through inference of the object it belongs to. A deprecated function of a module is `py/deprecated-method`'s.
