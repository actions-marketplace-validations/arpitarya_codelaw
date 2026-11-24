# py/try-except-in-loop — cases

### case: a for loop on a 3.10 target

```py python=3.10
for raw in values:
    try:
        total += int(raw)
    except ValueError:
        pass
```

```expect
2:5 py/try-except-in-loop
```

### case: a while loop

```py python=3.9
while queue:
    try:
        run(queue.pop())
    except KeyError:
        continue
```

```expect
2:5 py/try-except-in-loop
```

### case: a 3.11 target

```py python=3.11
for raw in values:
    try:
        total += int(raw)
    except ValueError:
        pass
```

```expect
(none)
```

### case: no target

```py
for raw in values:
    try:
        total += int(raw)
    except ValueError:
        pass
```

```expect
(none)
```

### case: a break in the try body is the loop's control flow

```py python=3.10
for raw in values:
    try:
        break
    except ValueError:
        pass
```

```expect
(none)
```

### case: a break of an inner loop does not count

```py python=3.10
for raw in values:
    try:
        for x in raw:
            break
    except ValueError:
        pass
```

```expect
2:5 py/try-except-in-loop
```

### case: the try is not the first statement

```py python=3.10
for raw in values:
    print(raw)
    try:
        total += int(raw)
    except ValueError:
        pass
```

```expect
(none)
```

### case: a try with only a finally

```py python=3.10
for raw in values:
    try:
        total += int(raw)
    finally:
        done()
```

```expect
(none)
```
