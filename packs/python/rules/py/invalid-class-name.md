---
id: py/invalid-class-name
title: "Class names are CapWords"
languages: [python]
fix: suggest
fix_reason: "suggest for a private or local class, renamed with its uses in this file; a public one is manual, since its importers would break"
fix_exception: restructure
stability: preview
ported_from: [ruff/N801, pylint/invalid-name]
since: 0.5.0
---

A class is named in CapWords: it starts with a capital letter, after any leading underscores, and has no underscore after them.

**Why:** PEP 8 names classes in CapWords, so a reader tells a class from a function or a variable at the call site.

```codelaw
match:
  kind: identifier
  pattern: $N
  inside: {kind: class_definition, field: name}
  any:
    - regex: '^_*[a-z]'
    - regex: '^_*[A-Za-z0-9]+_'
transform:
  U: { replace: { source: $N, replace: '^(_*).*$', by: '$1' } }
  R: { convert: { source: $N, toCase: pascalCase } }
message: "Name the class in CapWords: `$U$R`"
fix:
  - rename-symbol-local: {target: $N, to: $U$R}
```

## Examples

### Python

#### Don't

```python
class _user_record:
    pass


r = _user_record()
```

#### Do

```python
class _UserRecord:
    pass


r = _UserRecord()
```
