# py/redundant-literal-union — cases

### case: no matching type, a different numeric type, a type inside another generic

```py
a: Literal["foo"]
b: Union[Literal[5], Literal["foo"]]
c: Literal[True] | int
d: Literal[5] | float
e: dict[str, int] | Literal["x"]
```

```expect
(none)
```

### case: Union with bytes, an int inside a nested union, and a bool

```py
f: Union[Literal[b"a"], bytes]
g = Union[Literal[5], int]
h: bool | Literal[False]
```

```expect
1:18 py/redundant-literal-union
2:19 py/redundant-literal-union
3:19 py/redundant-literal-union
```
