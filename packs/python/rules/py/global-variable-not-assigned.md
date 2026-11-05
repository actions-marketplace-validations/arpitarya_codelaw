---
id: py/global-variable-not-assigned
title: "No global for a name never assigned"
languages: [python]
fix: suggest
fix_reason: "the declaration is removed; reading a global needs no `global`"
stability: preview
ported_from: [ruff/PLW0602]
since: 0.5.0
---

A `global` statement in a function names only variables the function assigns.

**Why:** Reading a module variable needs no `global`; the statement suggests a write that never happens.

```codelaw
match:
  kind: global_statement
  regex: '^global\s+\w+$'
  has: {kind: identifier, pattern: $N}
  inside:
    kind: function_definition
    stopBy: end
    not: {has: {stopBy: end, any: [{kind: assignment, has: {field: left, pattern: $N}}, {kind: augmented_assignment, has: {field: left, pattern: $N}}, {kind: for_statement, has: {field: left, pattern: $N}}, {kind: delete_statement, has: {pattern: $N}}, {kind: as_pattern_target, pattern: $N}, {kind: import_from_statement}, {kind: import_statement}]}}
message: "`$N` is never assigned here; `global` is not needed"
fix: [remove]
```

## Examples

### Python

#### Don't

```python
def show():
    global config
    print(config)
```

#### Do

```python
def show():
    print(config)
```
