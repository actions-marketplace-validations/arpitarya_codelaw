### case: an elif is rewritten by hand

```python
def sign(x):
    if x < 0:
        return -1
    elif x > 0:
        return 1
    return 0
```

```expect
4:5 py/superfluous-else-return
```
