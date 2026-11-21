# py/shebang-missing-executable-file — cases

A header ending `+x` gives the file the executable bit; a file with no header has none to read.

### case: an executable file with no shebang

```py
# run.py +x
print(1)
```

```expect
run.py:1:1 py/shebang-missing-executable-file
```

### case: an executable file with a shebang passes

```py
# run.py +x
#!/usr/bin/env python3
print(1)
```

```expect
```

### case: a file that is not executable passes

```py
# lib.py
print(1)
```

```expect
```

### case: an empty executable file is reported too

```py
# run.py +x
```

```expect
run.py:1:1 py/shebang-missing-executable-file
```

### case: a file checked alone has no bit to read

```py
print(1)
```

```expect
(none)
```
