---
id: py/disallowed-name
title: "No placeholder names"
languages: [python]
fix: manual
fix_reason: "the meaningful name is the author's"
fix_exception: value
stability: preview
ported_from: [pylint/disallowed-name]
since: 0.5.0
---

A variable, function, class or parameter is not named `foo`, `bar`, `baz`, `toto`, `tutu` or `tata`.

**Why:** A placeholder name says nothing about what the value is.

```codelaw
match:
  kind: identifier
  regex: '^(foo|bar|baz|toto|tutu|tata)$'
  any:
    - inside: {kind: assignment, field: left}
    - inside: {kind: function_definition, field: name}
    - inside: {kind: class_definition, field: name}
    - inside: {kind: parameters}
    - inside: {kind: for_statement, field: left}
message: "Give it a name that says what it holds"
```

## How to fix

1. Rename it after what it holds or does.

## Examples

### Python

#### Don't

```python
foo = load()
```

#### Do

```python
config = load()
```
