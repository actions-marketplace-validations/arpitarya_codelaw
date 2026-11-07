### case: None in the middle moves to the end

```python
x: int | None | str = 1
```

```expect
1:4 py/none-not-at-end-of-union
```

```fixed
x: int | str | None = 1
```
