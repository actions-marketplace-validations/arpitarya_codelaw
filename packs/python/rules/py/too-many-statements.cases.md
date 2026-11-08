# py/too-many-statements — cases

### case: an if counts with its body

```py options={"max":1}
def run():
    if a:
        b()
```

```expect
1:5 py/too-many-statements
```
