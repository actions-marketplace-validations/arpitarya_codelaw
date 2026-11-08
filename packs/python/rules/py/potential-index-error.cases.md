# py/potential-index-error — cases

### case: a negative index

```py
x = [1, 2][-3]
```

```expect
1:12 py/potential-index-error
```

### case: in range is fine

```py
x = [1, 2][-2]
y = (*a, 1)[5]
```

```expect
(none)
```
