---
id: py/eval-used
title: "No eval()"
languages: [python]
fix: manual
fix_reason: "a mechanical replacement could keep the injection; what the string should be allowed to contain is the author's"
fix_exception: security
stability: preview
ported_from: [pylint/eval-used]
since: 0.5.0
---

The builtin `eval()` is not called.

**Why:** `eval` runs any expression it is given; with input from outside, that is code injection.

```codelaw
match:
  kind: call
  has: {field: function, kind: identifier, regex: '^eval$', pattern: $F}
where:
  $F: builtin
message: "Do not evaluate strings as code"
```

## How to fix

1. Parse the input instead: `ast.literal_eval` for literals, `json.loads` for data, a dispatch table for names.

## Examples

### Python

#### Don't

```python
value = eval(text)
```

#### Do

```python
value = ast.literal_eval(text)
```
