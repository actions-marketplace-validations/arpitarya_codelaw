# py/invalid-module-name — cases

### case: a dash

```py path=order-lines/__init__.py
x = 1
```

```expect
1:1 py/invalid-module-name
```
