---
id: py/useless-else-on-loop
title: "No else on a loop without break"
languages: [python]
fix: manual
fix_reason: "dedenting the `else` body re-indents it, which a template edit cannot do"
fix_exception: restructure
stability: preview
ported_from: [ruff/PLW0120]
since: 0.5.0
---

A `for` or `while` loop has an `else` only when its body can `break`.

**Why:** Without a `break` the `else` always runs; its body may as well follow the loop.

```codelaw
match:
  kind: else_clause
  inside: {any: [{kind: for_statement}, {kind: while_statement}], not: {has: {field: body, has: {stopBy: end, kind: break_statement}}}}
message: "The loop never breaks, so `else` always runs; dedent its body"
```

## How to fix

1. Remove `else:` and dedent its body to follow the loop.

## Examples

### Python

#### Don't

```python
for x in xs:
    print(x)
else:
    done()
```

#### Do

```python
for x in xs:
    print(x)
done()
```
