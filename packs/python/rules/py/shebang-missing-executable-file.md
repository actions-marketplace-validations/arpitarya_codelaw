---
id: py/shebang-missing-executable-file
title: "An executable file has a shebang"
languages: [python]
fix: manual
fix_reason: "whether the file is a script (add the shebang) or only a module (drop the bit) is the author's"
fix_exception: value
stability: preview
ported_from: [ruff/EXE002]
since: 0.5.0
---

A Python file with the executable bit set starts with a `#!` shebang.

The bit is read from the file's mode (`FileContext::is_executable`, SR-ARCHITECTURE decision 15b). Where there is no mode to read — on Windows, or for a file checked alone — the rule reports nothing, on every machine alike; git's recorded mode is not consulted.

**Why:** Run directly, an executable file with no shebang is handed to the shell, not to Python.

```codelaw
native: py/shebang-missing-executable-file
```

## How to fix

1. Add `#!/usr/bin/env python3` as the first line, or
2. clear the executable bit (`chmod -x lib.py`) when the file is only imported.

## Examples

### Python

#### Don't

```python
# run.py +x
print(1)
```

#### Do

```python
# run.py +x
#!/usr/bin/env python3
print(1)
```
