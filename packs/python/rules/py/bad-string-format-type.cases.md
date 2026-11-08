# py/bad-string-format-type — cases

### case: a string for %d

```py
print('%d items' % 'three')
```

```expect
1:7 py/bad-string-format-type
```

### case: a tuple

```py
print('%s: %d' % ('a', 'b'))
```

```expect
1:7 py/bad-string-format-type
```

### case: a dict

```py
print('%(n)d' % {'n': 'x'})
```

```expect
1:7 py/bad-string-format-type
```

### case: None for %f and a float for %x

```py
a = '%f' % None
b = '%x' % 1.5
```

```expect
1:5 py/bad-string-format-type
2:5 py/bad-string-format-type
```

### case: %s takes anything, %d takes an int or float

```py
a = '%s' % None
b = '%d' % 3
c = '%d' % 2.5
d = '%f' % 1
```

```expect
(none)
```

### case: %c with a single character

```py
a = '%c' % 'x'
b = '%c' % 65
```

```expect
(none)
```

### case: a non-literal argument

```py
a = '%d' % value
b = '%d %d' % pair
```

```expect
(none)
```

### case: more conversions than arguments is another error

```py
a = '%d %d' % ('x',)
```

```expect
(none)
```
