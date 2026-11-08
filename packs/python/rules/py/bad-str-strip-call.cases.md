# py/bad-str-strip-call — cases

### case: the suggestion keeps each character once

```py
'x'.strip('aab')
```

```expect
1:11 py/bad-str-strip-call
```

```fixed
'x'.strip('ab')
```

### case: a receiver of unknown type is not read

```py
def f(x):
    return x.strip('aa')
```

```expect
(none)
```
