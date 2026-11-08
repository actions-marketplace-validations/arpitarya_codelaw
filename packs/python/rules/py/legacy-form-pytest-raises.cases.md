### case: inside a test, the body is indented under the with

```python
def test_parse():
    pytest.raises(ValueError, parse, "x", strict)
```

```expect
2:5 py/legacy-form-pytest-raises
```

```fixed
def test_parse():
    with pytest.raises(ValueError):
        parse("x", strict)
```
