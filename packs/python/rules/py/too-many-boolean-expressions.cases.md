# py/too-many-boolean-expressions — cases

### case: an elif

```py options={"max":2}
if x:
    pass
elif a or b or c:
    pass
```

```expect
3:6 py/too-many-boolean-expressions
```
