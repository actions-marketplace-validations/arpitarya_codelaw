---
id: py/duplicate-string-formatting-argument
title: "`str.format` gets each name once"
languages: [python]
fix: manual
fix_reason: "numbering the fields so one argument fills several rewrites the format string"
fix_exception: restructure
stability: preview
ported_from: [pylint/duplicate-string-formatting-argument]
since: 0.5.0
---

A `"…".format(...)` call on a string literal passes the same name twice as a positional argument: `"{} {} {}".format(a, a, b)`. Number the fields instead, `"{0} {0} {1}".format(a, b)`. Only bare names are compared; a repeated literal or attribute is left alone, and so is a call with `*args` or `**kwargs`.

**Why:** Passing a value twice hides that two fields show the same thing, and a later edit changes one and not the other.

```codelaw
match:
  kind: call
  pattern: $S.format($$$A)
  all:
    - has:
        field: arguments
        has:
          kind: identifier
          pattern: $N
          precedes: {stopBy: end, kind: identifier, pattern: $N}
    - not: {has: {field: arguments, has: {any: [{kind: list_splat}, {kind: dictionary_splat}]}}}
constraints:
  S: {kind: string}
message: "`$N` is passed to `format` more than once; number the fields instead"
```

## How to fix

1. Give each field an index or a name, and pass each value once.

## Examples

### Python

#### Don't

```python
line = "{} sent {} to {}".format(user, amount, user)
```

#### Do

```python
line = "{0} sent {1} to {0}".format(user, amount)
```
