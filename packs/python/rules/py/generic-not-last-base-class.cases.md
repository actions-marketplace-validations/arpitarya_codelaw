# py/generic-not-last-base-class — cases

### case: Generic last, or alone, is fine

```py
class A(Sized, Generic[T]):
    pass
class B(Generic[T]):
    pass
class C(Generic[T], metaclass=Meta):
    pass
```

```expect
(none)
```

### case: before a base and a keyword

```py
class D(Generic[T], str, metaclass=type):
    pass
```

```expect
1:8 py/generic-not-last-base-class
```
