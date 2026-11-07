---
id: py/mutable-dataclass-default
title: "No mutable default in a dataclass field"
languages: [python]
fix: suggest
fix_reason: "each instance gets its own value, built by `default_factory`; an empty list, dict or set is built by its type, any other by a `lambda`; `field` is imported from `dataclasses` where the file does not"
stability: preview
ported_from: [ruff/RUF008]
since: 0.5.0
---

A dataclass field defaults to a fresh list, dict or set through `field(default_factory=…)`, not a literal.

**Why:** A literal default is shared by every instance; dataclasses refuse `list`, `dict` and `set` literals at runtime.

```codelaw
match:
  kind: assignment
  has: {field: type, kind: type}
  all:
    - has: {field: right, any: [{kind: list}, {kind: dictionary}, {kind: set}], pattern: $V}
    - any:
        - has: {field: right, kind: list, regex: '^\[\s*\]$', pattern: $LIST}
        - has: {field: right, kind: dictionary, regex: '^\{\s*\}$', pattern: $DICT}
        - has: {field: right, not: {regex: '^(\[\s*\]|\{\s*\})$'}, pattern: $OTHER}
    - inside:
        kind: class_definition
        stopBy: end
        inside: {kind: decorated_definition, has: {kind: decorator, regex: '^@(dataclasses\.)?dataclass\b'}}
    - not: {inside: {kind: function_definition, stopBy: end}}
message: "This default is shared by every instance; use `field(default_factory=...)`"
fix:
  - replace: { target: $LIST, with: "field(default_factory=list)" }
  - replace: { target: $DICT, with: "field(default_factory=dict)" }
  - replace: { target: $OTHER, with: "field(default_factory=lambda: $OTHER)" }
  - ensure-import: { module: dataclasses, name: field }
```

## How to fix

1. Replace the literal with `field(default_factory=list)` (or `dict`, `set`, or a lambda returning the literal).

## Examples

### Python

#### Don't

```python
@dataclass
class Cart:
    items: list = []
```

#### Do

```python
from dataclasses import field

@dataclass
class Cart:
    items: list = field(default_factory=list)
```
