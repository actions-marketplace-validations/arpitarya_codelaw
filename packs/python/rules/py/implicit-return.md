---
id: py/implicit-return
title: "A function that returns a value ends with an explicit return"
languages: [python]
fix: auto
fix_reason: "adds `return None` where the end of the function can be reached, which changes nothing; a finding whose last statement shares its line with other code is reported without an edit"
stability: preview
ported_from: [ruff/RET503]
since: 0.5.0
---

A function with a `return` of a value other than `None` does not fall off its end: where the last statement can be left without returning (an `if` without `else`, a loop, a `with` whose body does not return), it ends with `return None`. Paths that cannot reach the end are not reported: `raise`, `try`, `while True`, `assert False`, a call to `sys.exit`, `exit`, `quit`, `os._exit`, `os.abort`, `pytest.fail`, `pytest.skip`, `pytest.xfail`, `typing.assert_never`, or to a function of the file annotated `NoReturn`/`Never`. Generators and a function of one `return` statement are skipped.

**Why:** A function that returns a value on some paths and silently returns `None` on others is easy to misread; the explicit `return None` says it is on purpose.

```codelaw
native: py/implicit-return
```

## Examples

### Python

#### Don't

```python
def check(x):
    if x:
        return 1
```

#### Do

```python
def check(x):
    if x:
        return 1
    return None
```
