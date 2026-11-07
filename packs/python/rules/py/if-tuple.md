---
id: py/if-tuple
title: "`if` doesn't test a tuple"
languages: [python]
fix: suggest
fix_reason: "the choices are testing the first item or all of them joined with `and`; the condition is always true today, so either changes what runs. A tuple with a starred item or a comment is manual"
fix_exception: value
stability: preview
ported_from: [ruff/F634]
since: 0.1.0
---

An `if` or `elif` condition isn't a non-empty tuple.

**Why:** A non-empty tuple is always true, so the branch always runs.

```codelaw
native: py/if-tuple
```

## How to fix

1. Write the condition you meant, usually the tuple's first element.

## Examples

### Python

#### Don't

```py
if (ready, "waiting"):
    start()
```

#### Fixed (first item)

```py
if ready:
    start()
```

#### Fixed (all items)

```py
if ready and "waiting":
    start()
```

#### Do

```py
if ready:
    start()
```
