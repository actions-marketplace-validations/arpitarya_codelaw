# py/pytest-parametrize-names-wrong-type — cases

### case: a list of names

```py
@pytest.mark.parametrize(["a", "b"], [(1, 2)])
def test_add(a, b):
    pass
```

```expect
1:26 py/pytest-parametrize-names-wrong-type
```

```fixed
@pytest.mark.parametrize(("a", "b"), [(1, 2)])
def test_add(a, b):
    pass
```

### case: a single-quoted string is reported without a fix

```py
@pytest.mark.parametrize('a, b', [(1, 2)])
def test_add(a, b):
    pass
```

```expect
1:26 py/pytest-parametrize-names-wrong-type
```
