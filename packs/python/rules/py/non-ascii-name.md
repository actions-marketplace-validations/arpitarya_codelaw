---
id: py/non-ascii-name
title: "Names are ASCII"
languages: [python]
fix: manual
fix_reason: "the ASCII spelling is the author's"
fix_exception: value
stability: preview
ported_from: [ruff/PLC2401, pylint/non-ascii-name]
since: 0.5.0
---

A name a statement binds — a variable, function, class or parameter — is spelt in ASCII.

**Why:** A non-ASCII name may look like another name, and is hard to type for anyone without that keyboard.

```codelaw
match:
  kind: identifier
  regex: '[^\x00-\x7F]'
  any:
    - inside: {kind: assignment, field: left}
    - inside: {kind: function_definition, field: name}
    - inside: {kind: class_definition, field: name}
    - inside: {kind: parameters}
    - inside: {kind: for_statement, field: left}
message: "Spell the name in ASCII"
```

## How to fix

1. Rename it with ASCII letters and update its uses.

## Examples

### Python

#### Don't

```python
größe = 3
```

#### Do

```python
groesse = 3
```
