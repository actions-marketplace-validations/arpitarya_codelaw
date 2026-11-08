# py/too-many-return-statements — cases

### case: over the limit

```py options={"max":2}
def grade(n):
    if n > 90:
        return 1
    if n > 80:
        return 2
    return 3
```

```expect
1:5 py/too-many-return-statements
```
