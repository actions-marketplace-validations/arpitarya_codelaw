# py/shebang-not-executable — cases

A header ending `+x` gives the file the executable bit; a file with no header has none to read.

### case: a shebang in a file that is not executable

```py
# run.py
#!/usr/bin/env python3
print(1)
```

```expect
run.py:1:1 py/shebang-not-executable
```

### case: an executable file passes

```py
# run.py +x
#!/usr/bin/env python3
print(1)
```

```expect
```

### case: no shebang, no finding

```py
# lib.py
print(1)
```

```expect
```

### case: a shebang that is not on the first line is another rule's

```py
# run.py
print(1)
#!/usr/bin/env python3
```

```expect
```

### case: a file checked alone has no bit to read

```py
#!/usr/bin/env python3
print(1)
```

```expect
(none)
```
