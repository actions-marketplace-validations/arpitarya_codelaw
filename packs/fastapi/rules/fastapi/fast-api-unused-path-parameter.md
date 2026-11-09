---
id: fastapi/fast-api-unused-path-parameter
title: "A route's path parameter is a parameter of the function"
languages: [python]
fix: suggest
fix_reason: "the parameter is added without an annotation, and where FastAPI reads it from (path, query, a dependency) is the author's to confirm; a positional-only parameter is reported without a fix"
stability: preview
ported_from: [ruff/FAST003]
since: 0.5.0
---

A route decorator (`@app.get("/items/{item_id}")` on a FastAPI app or an `APIRouter`: `get`, `post`, `put`, `delete`, `patch`, `options`, `head`, `trace`) whose path has a `{name}` or `{name:converter}` placeholder is on a function that takes `name`: as a parameter, as an `Annotated[…, Path(alias="name")]` alias, or through a `Depends(…)` dependency whose function or class (of the same file) takes it.

**Why:** FastAPI passes a path parameter only to a parameter of the same name; one that the function does not take is never read, and a typo in either name is a request that always fails with a 422 or a handler that ignores part of the URL.

```codelaw
native: fastapi/fast-api-unused-path-parameter
```


## Examples

### Python

#### Don't

```python
from fastapi import FastAPI

app = FastAPI()


@app.get("/items/{item_id}")
async def read_item(q: str):
    return {"q": q}
```

#### Do

```python
from fastapi import FastAPI

app = FastAPI()


@app.get("/items/{item_id}")
async def read_item(q: str, item_id: int):
    return {"q": q, "item_id": item_id}
```

#### Fixed

```python
from fastapi import FastAPI

app = FastAPI()


@app.get("/items/{item_id}")
async def read_item(q: str, item_id):
    return {"q": q}
```

## Differences from the source

- The app is a name assigned from `FastAPI(…)` or `APIRouter(…)`, or annotated as one, bound once in the file.
- A dependency this rule cannot read (not defined in the file, `Depends` twice on one parameter) ends the check for the route, as upstream.
- The fix is added after the last parameter without a default, before the first parameter when all have defaults, or in empty parentheses; the other shapes (keyword-only, `*args` only, positional-only) are reported without a fix.
