---
id: py/shebang-not-executable
title: "A file with a shebang is executable"
languages: [python]
fix: manual
fix_reason: "whether the file is a script (`chmod +x`) or only a module (drop the shebang) is the author's"
fix_exception: value
stability: preview
ported_from: [ruff/EXE001]
since: 0.5.0
---

A Python file whose first line is a `#!` shebang has the executable bit set.

The bit is read from the file's mode (`FileContext::is_executable`, SR-ARCHITECTURE decision 15b). Where there is no mode to read — on Windows, or for a file checked alone — the rule reports nothing, on every machine alike; git's recorded mode is not consulted.

**Why:** A shebang is read only when the file is run directly, which needs the executable bit; without it the line does nothing and misleads the reader.

```codelaw
native: py/shebang-not-executable
```

## How to fix

1. Make the file executable (`chmod +x run.py`; `git update-index --chmod=+x run.py` where the file system keeps no bit), or
2. remove the shebang line when the file is only imported.

## Examples

### Python

#### Don't

```python
# run.py
#!/usr/bin/env python3
print(1)
```

#### Do

```python
# run.py
print(1)
```

A script keeps its shebang and takes the bit (`# run.py +x` in a case).
