---
id: py/builtin-argument-shadowing
title: "No parameter named after a builtin"
languages: [python]
fix: suggest
fix_reason: "a caller that passes the argument by keyword must change too, so the rename is reviewed"
fix_exception: value
stability: preview
options: { ignorelist: { type: "string[]", default: [] } }
ported_from: [ruff/A002, pylint/redefined-builtin]
since: 0.5.0
---

A function's parameter is not named after a Python builtin (`id`, `type`, `filter`, …). A method marked `@override` is not reported, nor a name in `ignorelist`; a lambda's parameters are `py/builtin-lambda-argument-shadowing`'s.

**Why:** The builtin is hidden inside the function, and a later call to it calls the argument.

```codelaw
match:
  kind: identifier
  pattern: $N
  regex: '^(ArithmeticError|AssertionError|AttributeError|BaseException|BaseExceptionGroup|BlockingIOError|BrokenPipeError|BufferError|BytesWarning|ChildProcessError|ConnectionAbortedError|ConnectionError|ConnectionRefusedError|ConnectionResetError|DeprecationWarning|EOFError|EncodingWarning|EnvironmentError|Exception|ExceptionGroup|FileExistsError|FileNotFoundError|FloatingPointError|FutureWarning|GeneratorExit|IOError|ImportError|ImportWarning|IndentationError|IndexError|InterruptedError|IsADirectoryError|KeyError|KeyboardInterrupt|LookupError|MemoryError|ModuleNotFoundError|NameError|NotADirectoryError|NotImplementedError|OSError|OverflowError|PendingDeprecationWarning|PermissionError|ProcessLookupError|PythonFinalizationError|RecursionError|ReferenceError|ResourceWarning|RuntimeError|RuntimeWarning|StopAsyncIteration|StopIteration|SyntaxError|SyntaxWarning|SystemError|SystemExit|TabError|TimeoutError|TypeError|UnboundLocalError|UnicodeDecodeError|UnicodeEncodeError|UnicodeError|UnicodeTranslateError|UnicodeWarning|UserWarning|ValueError|Warning|ZeroDivisionError|abs|aiter|all|anext|any|ascii|bin|bool|breakpoint|bytearray|bytes|callable|chr|classmethod|compile|complex|copyright|credits|delattr|dict|dir|divmod|enumerate|eval|exec|exit|filter|float|format|frozenset|getattr|globals|hasattr|hash|help|hex|id|input|int|isinstance|issubclass|iter|len|license|list|locals|map|max|memoryview|min|next|object|oct|open|ord|pow|print|property|quit|range|repr|reversed|round|set|setattr|slice|sorted|staticmethod|str|sum|super|tuple|type|vars|zip)$'
  any:
    - inside: {kind: parameters}
    - inside: {kind: default_parameter, field: name, inside: {kind: parameters}}
    - inside: {kind: typed_default_parameter, field: name, inside: {kind: parameters}}
    - inside: {kind: typed_parameter, inside: {kind: parameters}}
    - inside: {kind: list_splat_pattern, inside: {any: [{kind: parameters}, {kind: typed_parameter}]}}
    - inside: {kind: dictionary_splat_pattern, inside: {any: [{kind: parameters}, {kind: typed_parameter}]}}
  not:
    inside:
      kind: function_definition
      stopBy: end
      inside: {kind: decorated_definition, has: {kind: decorator, regex: '^@(typing\.)?override$'}}
where:
  $N: {not: {in: ignorelist}}
transform:
  T: { replace: { source: $N, replace: '^(.*)$', by: '${1}_' } }
message: "`$N` hides the builtin of that name"
fix:
  - rename-symbol-local: {target: $N, to: $T}
```

## Examples

### Python

#### Don't

```python
def lookup(id, type="user"):
    return store[type][id]
```

#### Do

```python
def lookup(id_, type_="user"):
    return store[type_][id_]
```
