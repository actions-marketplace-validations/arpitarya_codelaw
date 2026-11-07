---
id: py/non-ascii-import-name
title: "Imported names are ASCII"
languages: [python]
fix: manual
fix_reason: "the ASCII alias is the author's"
fix_exception: value
stability: preview
ported_from: [ruff/PLC2403, pylint/non-ascii-module-import]
since: 0.5.0
---

An imported module or name, or its alias, is spelt in ASCII.

**Why:** A non-ASCII name may look like another name, and is hard to type for anyone without that keyboard.

```codelaw
match:
  kind: identifier
  regex: '[^\x00-\x7F]'
  any:
    - inside: {kind: aliased_import, field: alias}
    - inside: {kind: dotted_name, inside: {kind: import_from_statement, field: name}}
    - inside: {kind: dotted_name, inside: {kind: import_statement}}
message: "Import under an ASCII name"
```

## How to fix

1. Import it under an ASCII alias (`as size`).

## Examples

### Python

#### Don't

```python
from shapes import größe
```

#### Do

```python
from shapes import größe as size
```
