---
id: fastapi/fast-api-non-annotated-dependency
title: "Dependencies through Annotated"
languages: [python]
fix: suggest
fix_reason: "the suggestion uses `Annotated`, which the file must import from `typing`"
stability: preview
ported_from: [ruff/FAST002]
since: 0.5.0
---

A route parameter's `Depends`, `Query`, `Path`, `Body`, `Header`, `Cookie`, `Form`, `File` or `Security` is written in `Annotated[...]`, not as the default value.

**Why:** FastAPI recommends `Annotated`: the parameter keeps a real default, the function stays callable outside FastAPI, and the dependency can be reused as a type.

```codelaw
match:
  kind: typed_default_parameter
  all:
  - has: {field: name, pattern: $N}
  - has: {field: type, pattern: $T}
  - has: {field: value, kind: call, regex: '^(fastapi\.)?(Depends|Query|Path|Body|Header|Cookie|Form|File|Security)\(', pattern: $V}
message: 'Write `$N: Annotated[$T, …]` instead of a `$V` default'
fix: '$N: Annotated[$T, $V]'
```

## Examples

### Python

#### Don't

```python
async def read(db: Session = Depends(get_db)):
    return db
```

#### Do

```python
async def read(db: Annotated[Session, Depends(get_db)]):
    return db
```
