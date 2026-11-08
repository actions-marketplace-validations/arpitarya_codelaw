# py/hardcoded-string-charset — cases

### case: hexdigits in single quotes, an existing import

```py
import string

ok = set('0123456789abcdefABCDEF')
```

```expect
3:10 py/hardcoded-string-charset
```

```fixed
import string

ok = set(string.hexdigits)
```

### case: whitespace as repr spells it

```py
SEP = ' \t\n\r\x0b\x0c'
```

```expect
1:7 py/hardcoded-string-charset
```

### case: another order, bytes, a docstring and a concatenation are fine

```py
"""0123456789"""
a = "9876543210"
b = b"0123456789"
c = "01234" "56789"
d = f"0123456789{x}"
```

```expect
(none)
```
