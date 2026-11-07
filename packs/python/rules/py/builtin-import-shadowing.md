---
id: py/builtin-import-shadowing
title: "No import named after a builtin"
languages: [python]
fix: manual
fix_reason: "the alias to import it under is the author's"
fix_exception: value
stability: preview
options: { ignorelist: { type: "string[]", default: [] } }
ported_from: [ruff/A004]
since: 0.5.0
---

An import does not bind a builtin's name: `from os import open` and `import json as input` are reported. A name in `ignorelist` is not.

**Why:** The builtin is hidden for the whole module, and `open(...)` then calls `os.open`, which takes other arguments.

```codelaw
match:
  kind: identifier
  pattern: $N
  regex: '^(ArithmeticError|AssertionError|AttributeError|BaseException|BaseExceptionGroup|BlockingIOError|BrokenPipeError|BufferError|BytesWarning|ChildProcessError|ConnectionAbortedError|ConnectionError|ConnectionRefusedError|ConnectionResetError|DeprecationWarning|EOFError|EncodingWarning|EnvironmentError|Exception|ExceptionGroup|FileExistsError|FileNotFoundError|FloatingPointError|FutureWarning|GeneratorExit|IOError|ImportError|ImportWarning|IndentationError|IndexError|InterruptedError|IsADirectoryError|KeyError|KeyboardInterrupt|LookupError|MemoryError|ModuleNotFoundError|NameError|NotADirectoryError|NotImplementedError|OSError|OverflowError|PendingDeprecationWarning|PermissionError|ProcessLookupError|PythonFinalizationError|RecursionError|ReferenceError|ResourceWarning|RuntimeError|RuntimeWarning|StopAsyncIteration|StopIteration|SyntaxError|SyntaxWarning|SystemError|SystemExit|TabError|TimeoutError|TypeError|UnboundLocalError|UnicodeDecodeError|UnicodeEncodeError|UnicodeError|UnicodeTranslateError|UnicodeWarning|UserWarning|ValueError|Warning|ZeroDivisionError|abs|aiter|all|anext|any|ascii|bin|bool|breakpoint|bytearray|bytes|callable|chr|classmethod|compile|complex|copyright|credits|delattr|dict|dir|divmod|enumerate|eval|exec|exit|filter|float|format|frozenset|getattr|globals|hasattr|hash|help|hex|id|input|int|isinstance|issubclass|iter|len|license|list|locals|map|max|memoryview|min|next|object|oct|open|ord|pow|print|property|quit|range|repr|reversed|round|set|setattr|slice|sorted|staticmethod|str|sum|super|tuple|type|vars|zip)$'
  any:
    - inside: {kind: dotted_name, inside: {kind: import_from_statement, field: name}}
    - inside: {kind: aliased_import, field: alias}
    - inside: {kind: dotted_name, regex: '^\w+$', inside: {kind: import_statement}}
where:
  $N: {not: {in: ignorelist}}
message: "The import `$N` hides the builtin of that name"
```

## How to fix

1. Import it under another name (`from os import open as os_open`), or import the module and qualify the call (`os.open`).

## Examples

### Python

#### Don't

```python
from os import open
```

#### Do

```python
from os import open as os_open
```
