# py/bad-exit-annotation — cases

### case: object and unannotated arguments are fine

```py
class Resource:
    def __exit__(self, exc_type: object, exc, tb: object) -> None: ...
```

```expect
(none)
```

### case: star-args annotated with something else

```py
class Resource:
    def __exit__(self, *args: int) -> None: ...
```

```expect
2:31 py/bad-exit-annotation
```

```fixed
class Resource:
    def __exit__(self, *args: object) -> None: ...
```

### case: too few arguments

```py
class Resource:
    def __exit__(self, exc_type) -> None: ...
```

```expect
2:17 py/bad-exit-annotation
```

### case: an async method

```py
class Resource:
    async def __aexit__(self, exc_type: int, exc: object, tb: object) -> None: ...
```

```expect
2:41 py/bad-exit-annotation
```
