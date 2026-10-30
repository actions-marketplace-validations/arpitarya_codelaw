# fastapi/fast-api-redundant-response-model — cases

### case: a different model is not redundant

```python
@app.get("/items", response_model=Public)
async def read() -> Item:
    return Item()
```

```expect

```
