### case: three conditions become three asserts

```python
assert a and b and c
```

```expect
1:1 py/pytest-composite-assertion
```

```fixed
assert a
assert b
assert c
```
