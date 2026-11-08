# py/invalid-argument-name — cases

### case: typed parameters and splats

```py
def f(userId: int, *Args, **KW):
    return userId, Args, KW
```

```expect
1:7 py/invalid-argument-name
1:21 py/invalid-argument-name
1:29 py/invalid-argument-name
```

### case: a lowercase parameter and a type annotation are not reported

```py
def f(user: User, n: int = MAX):
    return user
```

```expect
(none)
```
