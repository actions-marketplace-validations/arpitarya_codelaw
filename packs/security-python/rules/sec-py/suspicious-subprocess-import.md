---
id: sec-py/suspicious-subprocess-import
title: "`subprocess` is imported"
languages: [python]
fix: manual
fix_reason: "whether the import is a risk depends on what is passed to it; no edit makes it safe"
fix_exception: security
stability: preview
ported_from: [ruff/S404]
security: { cwe: [78], owasp: ["A05:2025"], severity: low }
since: 0.5.0
---

`import subprocess` and `from subprocess import …` are reported at the import, outside an `if TYPE_CHECKING:` block and outside stub files. Opt-in: a project that wants every use of a subprocess to be reviewed.

**Why:** A subprocess call is where a command line is built from data, and the injection rules (`sec-py/no-shell`) only see the call. Listing the import points a reviewer at each module that can start a process (CWE-78).

```codelaw
native: sec-py/suspicious-subprocess-import
```

## How to fix

1. Check that every call passes a list of arguments and no `shell=True`; `sec-py/no-shell` reports the ones that do not.
2. Keep the import where the call is needed, and the module's inputs validated.
3. If the module only needs a path or an environment lookup, use `pathlib` or `shutil` and drop the import.


## Examples

### Python

#### Don't

```python
import subprocess

subprocess.run(["ls", path])
```

#### Do

```python
import os

entries = os.listdir(path)
```

## Differences from the source

The import is found as written. Ruff's rule has no fix either.
