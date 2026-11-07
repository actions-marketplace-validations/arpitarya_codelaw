# Cases: py/import-shadowed-by-loop-var

### case: in a function, renamed

```py
def show(names):
    import path
    for path in names:
        print(path)
    return 1
```

```expect
3:9 py/import-shadowed-by-loop-var
```

```fixed
def show(names):
    import path
    for path_ in names:
        print(path_)
    return 1
```

### case: the new name is taken

```py
import path
path_ = 1
for path in ["a.txt"]:
    print(path, path_)
```

```expect
3:5 py/import-shadowed-by-loop-var
```
