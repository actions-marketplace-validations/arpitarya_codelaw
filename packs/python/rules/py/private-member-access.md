---
id: py/private-member-access
title: "No access to another object's private members"
languages: [python]
fix: manual
fix_reason: "what the owner should expose instead is a design choice"
fix_exception: restructure
stability: preview
options: { ignore_names: { type: "string[]", default: ["_make", "_asdict", "_replace", "_fields", "_field_defaults", "_name_", "_value_"] } }
ported_from: [ruff/SLF001, pylint/protected-access]
since: 0.5.0
---

An attribute whose name starts with one underscore is read or written only on `self`, `cls` or `mcs`, or through `super()`. Dunder names, and the names in `ignore_names` (namedtuple's and enum's public `_make`, `_asdict`, `_replace`, `_fields`, `_field_defaults`, `_name_`, `_value_` by default), are not reported.

**Why:** A leading underscore marks a member as the class's own; code outside it that relies on one breaks when the class changes.

```codelaw
match:
  kind: identifier
  pattern: $A
  regex: '^_[^_]|^_$|^__\w*[^_]_?$'
  inside:
    kind: attribute
    field: attribute
    not:
      has:
        field: object
        any:
          - {regex: '^(self|cls|mcs)$'}
          - {kind: call, regex: '^super\('}
where:
  $A: {not: {in: ignore_names}}
message: "`$A` is private to its class"
```

## How to fix

1. Use the object's public interface, or give its class a public method or property for what the code needs.

## Examples

### Python

#### Don't

```python
def total(cart):
    return cart._items_total()
```

#### Do

```python
def total(cart):
    return cart.items_total()
```
