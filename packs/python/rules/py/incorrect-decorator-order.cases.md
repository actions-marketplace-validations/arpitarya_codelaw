# py/incorrect-decorator-order — cases

### case: the right order, a decorator between, and pairs that do not clash

```py
class A:
    @property
    @abstractmethod
    def a(self): ...

    @abstractmethod
    @other
    @property
    def b(self): ...

    @contextmanager
    @staticmethod
    def c(): ...

    @abstractmethod
    @abstractproperty
    def d(self): ...
```

```expect
(none)
```

### case: a cache with arguments, and a setter

```py
class B:
    @functools.lru_cache(maxsize=1)
    @property
    def a(self): ...

    @abstractmethod
    @a.setter
    def a(self, v): ...
```

```expect
2:5 py/incorrect-decorator-order
6:5 py/incorrect-decorator-order
```
