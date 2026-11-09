---
id: sec-py/start-process-with-no-shell
title: "A process is started with `os.exec*`, `os.spawn*` or `os.startfile`"
languages: [python]
fix: manual
fix_reason: "starting the process is the point; whether its arguments are safe is the author's"
fix_exception: security
stability: preview
ported_from: [ruff/S606]
security: { cwe: [78], owasp: ["A05:2025"], severity: low }
since: 0.5.0
---

A call that resolves to `os.execl`, `execle`, `execlp`, `execlpe`, `execv`, `execve`, `execvp`, `execvpe`, `spawnl`, `spawnle`, `spawnlp`, `spawnlpe`, `spawnv`, `spawnve`, `spawnvp`, `spawnvpe` or `startfile` is reported. These start a process without a shell, which is a smaller risk than `os.system` (`sec-py/no-shell`).

**Why:** A process started with a value the program did not write runs whatever that value names. The `os` functions have no shell to inject into, so the risk is the program or the arguments an attacker can choose (CWE-78).

```codelaw
native: sec-py/start-process-with-no-shell
```

## How to fix

1. Pass a fixed program and validate every argument.
2. Use `subprocess.run([...])` with a list and `check=True`, which reports a failed start.


## Examples

### Python

#### Don't

```python
import os

os.execv(program, [program, name])
```

#### Do

```python
import subprocess

subprocess.run(["/usr/bin/tool", name], check=True)
```

## Differences from the source

Names are resolved through the file's imports (`from os import execv` counts); the rule reports the call, where Ruff reports the same range.
