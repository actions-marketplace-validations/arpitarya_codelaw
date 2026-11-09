# py/unused-private-type-alias — cases

### case: a type statement

```py
type _Number = int | float
```

```expect
1:6 py/unused-private-type-alias
```

### case: a plain assignment is not an alias

```py
_Number = int | float
```

```expect
(none)
```
