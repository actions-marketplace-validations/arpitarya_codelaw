# py/bad-chained-comparison — cases

### case: one kind of operator chains as written

```py
ok = a < b <= c
ok = a == b != c
ok = a is b is not c
ok = a in b not in c
```

```expect
(none)
```

### case: membership mixed with equality

```py
both = x in s == y in s
```

```expect
1:8 py/bad-chained-comparison
```

### case: two operands are a plain comparison

```py
ok = a is not None
```

```expect
(none)
```
