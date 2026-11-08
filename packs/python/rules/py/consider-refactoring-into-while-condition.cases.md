# py/consider-refactoring-into-while-condition — cases

### case: the break is not first, or the condition is real or falsy

```py
while True:
    step()
    if done:
        break
while k < 10:
    if k == 3:
        break
while 0:
    if x:
        break
while "":
    if x:
        break
```

```expect
(none)
```

### case: a non-zero number and a non-empty string

```py
while 1:
    if x:
        break
while "go":
    if y:
        break
```

```expect
1:1 py/consider-refactoring-into-while-condition
4:1 py/consider-refactoring-into-while-condition
```
