# py/unused-private-member — cases

### case: an unused method, class variable and instance attribute

```py
class A:
    __secret = 1

    def __init__(self):
        self.__value = 2

    def __helper(self, x):
        pass
```

```expect
2:5 py/unused-private-member
5:9 py/unused-private-member
7:5 py/unused-private-member
```

### case: used through self, cls, the class and type(self)

```py
class A:
    __a = 1
    __b = 2
    __c = 3
    __d = 4

    def m(self):
        return self.__a, A.__b, type(self).__c, self.__class__.__d
```

```expect
(none)
```

### case: a classmethod's store is not a use

```py
class A:
    __x = None

    @classmethod
    def set(cls, v):
        cls.__x = v
```

```expect
2:5 py/unused-private-member
6:9 py/unused-private-member
```

### case: a store and a read through different spellings

```py
class A:
    __x = None

    @classmethod
    def set(cls, v):
        cls.__x = v

    @classmethod
    def get(cls):
        return cls.__x
```

```expect
(none)
```

### case: a read on another object is not a use

```py
class A:
    def __test(self):
        pass

    def m(self, other):
        other.__test()
```

```expect
2:5 py/unused-private-member
```

### case: a function passed in the class body

```py
class A:
    def __get(self):
        pass

    prop = property(__get)
```

```expect
(none)
```

### case: a nested function used and unused

```py
class A:
    def m(self):
        def __used():
            pass

        def __unused():
            pass

        __used()
```

```expect
6:9 py/unused-private-member
```

### case: dunder names and single underscore are not private

```py
class A:
    _x = 1
    __slots__ = ()

    def __str__(self):
        return ''
```

```expect
(none)
```
