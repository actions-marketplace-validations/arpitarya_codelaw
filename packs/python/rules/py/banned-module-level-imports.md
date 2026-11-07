---
id: py/banned-module-level-imports
title: "No module-level import of the modules the project lists"
languages: [python]
fix: manual
fix_reason: "the import moves into the functions that use it"
fix_exception: restructure
stability: preview
options: { modules: { type: "string[]", default: [] } }
ported_from: [ruff/TID253]
since: 0.5.0
---

A module the project lists in `modules` — a heavy one, say `torch` — is imported inside the functions that use it, not at module level. The list is empty by default.

**Why:** A module-level import of a slow module costs every program that imports this one, used or not.

```codelaw
match:
  kind: dotted_name
  pattern: $M
  any:
    - inside: {kind: import_statement, inside: {kind: module}}
    - inside: {kind: aliased_import, field: name, inside: {kind: import_statement, inside: {kind: module}}}
    - inside: {kind: import_from_statement, field: module_name, inside: {kind: module}}
where:
  $M: {in: modules}
message: "Import `$M` where it is used, not at module level"
```

## How to fix

1. Move the import into the functions that use the module.

## Examples

### Python

#### Don't

```python options={"modules":["torch"]}
import torch
```

#### Do

```python options={"modules":["torch"]}
def train():
    import torch
    return torch.zeros(1)
```
