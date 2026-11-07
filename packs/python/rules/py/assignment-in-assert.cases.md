### case: the binding is the whole condition

```python
def check(items):
    assert (first := items[0])
    return first
```

```expect
2:13 py/assignment-in-assert
```

```fixed
def check(items):
    first = items[0]
    assert first
    return first
```

### case: a binding after an `and` is moved by hand

```python
assert ok and (n := len(items)) > 0
```

```expect
1:16 py/assignment-in-assert
```
