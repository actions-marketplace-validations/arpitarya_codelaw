# py/stdlib-module-shadowing — cases

### case: inside a package, with strict checking

```py path=app/json.py options={"strict_checking":true}
x = 1
```

```expect
1:1 py/stdlib-module-shadowing
```

### case: an allowed name

```py path=logging.py options={"allowed_modules":["logging"]}
x = 1
```

```expect
(none)
```
