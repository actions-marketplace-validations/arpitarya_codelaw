---
id: py/invalid-field-call
title: "dataclasses.field() only in a dataclass body"
languages: [python]
fix: manual
fix_reason: "the value moves into a dataclass, or the call is replaced by the value it was meant to make"
fix_exception: restructure
stability: preview
ported_from: [pylint/invalid-field-call]
since: 0.5.0
---

`dataclasses.field()` (or `field()` imported from it) is called only as the value of an annotated class attribute.

**Why:** Elsewhere `field()` returns an inert `Field` object and does nothing.

```codelaw
match:
  kind: call
  has: {field: function, regex: '^(dataclasses\.)?field$'}
  not:
    inside:
      kind: assignment
      field: right
      has: {field: type, kind: type}
      inside: {kind: expression_statement, inside: {kind: block, inside: {kind: class_definition}}}
where:
  $$MATCH: {imported-from: dataclasses}
message: "`field()` works only as a dataclass attribute's value"
```

## How to fix

1. Use `field()` only as an annotated attribute of a `@dataclass`; elsewhere, write the value.

## Examples

### Python

#### Don't

```python
from dataclasses import field

items = field(default_factory=list)
```

#### Do

```python
from dataclasses import dataclass, field


@dataclass
class Cart:
    items: list = field(default_factory=list)
```
