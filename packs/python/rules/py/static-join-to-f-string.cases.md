# py/static-join-to-f-string — cases

### case: only literals

```py
x = " ".join(["a", "b"])
```

```expect
1:5 py/static-join-to-f-string
```

```fixed
x = "a b"
```

### case: a generator is not a static list

```py
x = ",".join(str(i) for i in items)
```

```expect
(none)
```

### case: a starred element

```py
x = ",".join([*a, b])
```

```expect
(none)
```
