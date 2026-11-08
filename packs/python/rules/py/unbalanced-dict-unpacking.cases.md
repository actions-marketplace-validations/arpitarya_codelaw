# py/unbalanced-dict-unpacking — cases

### case: an assignment from a dict

```py
a, b = {"x": 1}
```

```expect
1:1 py/unbalanced-dict-unpacking
```

### case: .items() with three targets

```py
for a, b, c in {"x": 1}.items():
    pass
```

```expect
1:1 py/unbalanced-dict-unpacking
```
