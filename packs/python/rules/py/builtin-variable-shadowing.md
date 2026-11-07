---
id: py/builtin-variable-shadowing
title: "No variable named after a builtin"
languages: [python]
fix: suggest
fix_reason: "suggest for a local name, renamed with a trailing underscore along with its uses in this file; a public module-level name is manual, since importers use it"
fix_exception: value
stability: preview
options: { ignorelist: { type: "string[]", default: [] } }
ported_from: [ruff/A001, pylint/redefined-builtin]
since: 0.5.0
---

A variable, loop target, `with` or `except` target, or a function or class defined outside a class body, is not named after a Python builtin (`list`, `id`, `type`, `input`, …). Dunder names such as `__doc__` are not reported, and nor is a name in `ignorelist`.

**Why:** The builtin is hidden for the rest of the scope, and a later `list(...)` calls the variable.

```codelaw
utils:
  target:
    any:
      - inside: {kind: assignment, field: left}
      - inside: {kind: augmented_assignment, field: left}
      - inside: {any: [{kind: pattern_list}, {kind: tuple_pattern}], inside: {kind: assignment, field: left}}
      - inside: {any: [{kind: pattern_list}, {kind: tuple_pattern}], inside: {kind: for_statement, field: left}}
      - inside: {kind: for_statement, field: left}
      - inside: {kind: as_pattern_target}
      - inside: {kind: named_expression, field: name}
      - inside: {kind: function_definition, field: name}
      - inside: {kind: class_definition, field: name}
match:
  kind: identifier
  pattern: $N
  regex: '^(ArithmeticError|AssertionError|AttributeError|BaseException|BaseExceptionGroup|BlockingIOError|BrokenPipeError|BufferError|BytesWarning|ChildProcessError|ConnectionAbortedError|ConnectionError|ConnectionRefusedError|ConnectionResetError|DeprecationWarning|EOFError|EncodingWarning|EnvironmentError|Exception|ExceptionGroup|FileExistsError|FileNotFoundError|FloatingPointError|FutureWarning|GeneratorExit|IOError|ImportError|ImportWarning|IndentationError|IndexError|InterruptedError|IsADirectoryError|KeyError|KeyboardInterrupt|LookupError|MemoryError|ModuleNotFoundError|NameError|NotADirectoryError|NotImplementedError|OSError|OverflowError|PendingDeprecationWarning|PermissionError|ProcessLookupError|PythonFinalizationError|RecursionError|ReferenceError|ResourceWarning|RuntimeError|RuntimeWarning|StopAsyncIteration|StopIteration|SyntaxError|SyntaxWarning|SystemError|SystemExit|TabError|TimeoutError|TypeError|UnboundLocalError|UnicodeDecodeError|UnicodeEncodeError|UnicodeError|UnicodeTranslateError|UnicodeWarning|UserWarning|ValueError|Warning|ZeroDivisionError|abs|aiter|all|anext|any|ascii|bin|bool|breakpoint|bytearray|bytes|callable|chr|classmethod|compile|complex|copyright|credits|delattr|dict|dir|divmod|enumerate|eval|exec|exit|filter|float|format|frozenset|getattr|globals|hasattr|hash|help|hex|id|input|int|isinstance|issubclass|iter|len|license|list|locals|map|max|memoryview|min|next|object|oct|open|ord|pow|print|property|quit|range|repr|reversed|round|set|setattr|slice|sorted|staticmethod|str|sum|super|tuple|type|vars|zip)$'
  matches: target
  not:
    any:
      - inside: {kind: class_definition, stopBy: {any: [{kind: function_definition}, {kind: lambda}]}}
      - inside: {kind: function_definition, field: name, inside: {kind: block, inside: {kind: class_definition}}}
      - inside: {kind: class_definition, field: name, inside: {kind: block, inside: {kind: class_definition}}}
      - inside: {kind: function_definition, field: name, inside: {kind: decorated_definition, inside: {kind: block, inside: {kind: class_definition}}}}
      - inside: {kind: class_definition, field: name, inside: {kind: decorated_definition, inside: {kind: block, inside: {kind: class_definition}}}}
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
def count(items):
    list = sorted(items)
    return len(list)
```

#### Do

```python
def count(items):
    list_ = sorted(items)
    return len(list_)
```
