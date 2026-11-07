---
id: py/exec-used
title: "No exec()"
languages: [python]
fix: manual
fix_reason: "a mechanical replacement could keep the injection; what the code should do is the author's"
fix_exception: security
stability: preview
ported_from: [pylint/exec-used]
since: 0.5.0
---

The builtin `exec()` is not called.

**Why:** `exec` runs any code it is given; with input from outside, that is code injection.

```codelaw
match:
  kind: call
  has: {field: function, kind: identifier, regex: '^exec$', pattern: $F}
where:
  $F: builtin
message: "Do not run strings as code"
```

## How to fix

1. Replace the generated code with ordinary functions and a dispatch table.

## Examples

### Python

#### Don't

```python
exec(source)
```

#### Do

```python
handlers[name]()
```
