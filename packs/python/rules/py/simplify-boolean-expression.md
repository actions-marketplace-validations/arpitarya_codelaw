---
id: py/simplify-boolean-expression
title: "`c and <falsy> or v` is written `v`"
languages: [python]
fix: suggest
fix_reason: "the condition is no longer evaluated, which matters only if it has side effects"
stability: preview
ported_from: [pylint/simplify-boolean-expression]
since: 0.5.0
---

In an assignment or a `return`, `cond and False or value` always gives `value`: when `cond` is true the middle is falsy, and when it is false `cond` itself is. The middle operand here is a falsy literal — `False`, `None`, `0`, `""`, `()`, `[]`, `{}` — where Pylint also infers a falsy name; the old `and`/`or` ternary with a truthy middle is not reported.

**Why:** The expression looks like a choice between two values and is not one: the first branch can never be taken.

```codelaw
match:
  kind: boolean_operator
  pattern: $C and $F or $V
  any:
    - inside: {kind: assignment, field: right}
    - inside: {kind: return_statement}
constraints:
  C: {not: {kind: boolean_operator}}
  V: {not: {kind: boolean_operator}}
  F:
    any:
      - kind: "false"
      - kind: none
      - {kind: integer, regex: '^0+$'}
      - {kind: string, regex: '^("""|''''''|"|'')("""|''''''|"|'')$'}
      - {kind: tuple, regex: '^\(\s*\)$'}
      - {kind: list, regex: '^\[\s*\]$'}
      - {kind: dictionary, regex: '^\{\s*\}$'}
message: "This always gives `$V`"
fix: "$V"
```

## Examples

### Python

#### Don't

```python
mode = debug and None or default_mode
```

#### Do

```python
mode = default_mode
```
