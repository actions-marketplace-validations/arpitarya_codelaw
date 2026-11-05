---
id: py/del-attr-with-constant
title: "del obj.name, not delattr(obj, 'name')"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/B043]
since: 0.5.0
---

`delattr(obj, "name")` with a constant, valid attribute name is written `del obj.name`.

**Why:** `delattr` with a constant is no safer than `del`, only harder to read.

```codelaw
match:
  kind: expression_statement
  has: {kind: call, pattern: 'delattr($O, $S)', has: {stopBy: end, kind: string_content, pattern: $C}}
constraints:
  S: {regex: '^["''][A-Za-z_][A-Za-z0-9_]*["'']$', not: {regex: '^["'']__'}}
  O: {any: [{kind: identifier}, {kind: attribute}]}
message: "Write `del $O.$C`"
fix: "del $O.$C"
```

## Examples

### Python

#### Don't

```python
delattr(user, "name")
```

#### Do

```python
del user.name
```
