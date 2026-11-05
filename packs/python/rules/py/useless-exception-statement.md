---
id: py/useless-exception-statement
title: "Raise the exception you build"
languages: [python]
fix: suggest
fix_reason: "adding `raise` makes the line exit the function"
stability: preview
ported_from: [ruff/PLW0133, pylint/pointless-exception-statement]
since: 0.5.0
---

A statement that only constructs a built-in exception (`ValueError("...")`) raises it.

**Why:** Building an exception does nothing; `raise` was left out.

```codelaw
match:
  kind: expression_statement
  has: {kind: call, has: {field: function, regex: '^(BaseException|Exception|ArithmeticError|AssertionError|AttributeError|BufferError|EOFError|ImportError|ModuleNotFoundError|LookupError|IndexError|KeyError|MemoryError|NameError|NotImplementedError|OSError|OverflowError|RecursionError|ReferenceError|RuntimeError|StopIteration|SyntaxError|SystemError|TypeError|ValueError|UnicodeError|ZeroDivisionError|FileNotFoundError|FileExistsError|PermissionError|TimeoutError|ConnectionError)$'}}
message: "This exception is built but never raised"
fix: "raise $$MATCH"
```

## Examples

### Python

#### Don't

```python
def f(x):
    if x < 0:
        ValueError("negative")
```

#### Do

```python
def f(x):
    if x < 0:
        raise ValueError("negative")
```
