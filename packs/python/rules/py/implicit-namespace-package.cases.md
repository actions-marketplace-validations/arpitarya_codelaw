# py/implicit-namespace-package — cases

### case: a script with a shebang is fine

```py path=tools/run.py
#!/usr/bin/env python
print(1)
```

```expect
(none)
```

### case: a file directly in src/ is fine

```py path=src/manage.py
x = 1
```

```expect
(none)
```
