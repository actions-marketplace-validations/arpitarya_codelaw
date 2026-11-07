# py/used-prior-global-declaration — cases

### case: another name, a nested function, or module level are fine

```py
def a():
    print(other)
    global counter
    counter = 1

def b():
    def inner():
        return counter
    global counter
    counter = 2

total = 0
global total
```

```expect
(none)
```
