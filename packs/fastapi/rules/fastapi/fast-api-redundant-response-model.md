---
id: fastapi/fast-api-redundant-response-model
title: "No response_model that repeats the return annotation"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/FAST001]
since: 0.5.0
---

A route does not pass `response_model=` equal to its function's return annotation.

**Why:** FastAPI reads the response model from the return annotation; repeating it is noise that can drift.

```codelaw
match:
  kind: keyword_argument
  all:
  - has: {field: name, regex: ^response_model$}
  - has: {field: value, pattern: $T}
  - inside:
      kind: decorator
      stopBy: end
      regex: ^@\w+\.(get|post|put|patch|delete|head|options|trace|api_route)\(
      inside:
        kind: decorated_definition
        has:
          field: definition
          kind: function_definition
          has: {field: return_type, pattern: $T}
message: '`response_model` repeats the return annotation; remove it'
fix: [remove]
```

## Examples

### Python

#### Don't

```python
@app.get("/items", response_model=Item)
async def read() -> Item:
    return Item()
```

#### Do

```python
@app.get("/items")
async def read() -> Item:
    return Item()
```

#### Fixed

```python
@app.get("/items")
async def read() -> Item:
    return Item()
```
