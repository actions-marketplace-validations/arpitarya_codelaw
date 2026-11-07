# py/uncapitalized-environment-variables — cases

### case: a default value is not the name

```py
debug = os.environ.get("DEBUG", "false")
mode = os.getenv("MODE", "dev")
```

```expect
(none)
```

### case: getenv

```py
home = os.getenv("home")
```

```expect
1:19 py/uncapitalized-environment-variables
```

```fixed
home = os.getenv("HOME")
```
