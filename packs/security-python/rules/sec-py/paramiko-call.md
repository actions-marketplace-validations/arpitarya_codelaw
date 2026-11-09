---
id: sec-py/paramiko-call
title: "`paramiko.exec_command`"
languages: [python]
fix: manual
fix_reason: "what makes the command safe is the program's, not an edit"
fix_exception: security
stability: preview
ported_from: [ruff/S601]
security: { cwe: [78], owasp: ["A05:2025"], severity: medium }
since: 0.5.0
---

A call that resolves to `paramiko.exec_command` is reported.

**Why:** A command line sent to a remote shell is parsed there: a quote or `;` in any part of it runs another command (CWE-78).

```codelaw
native: sec-py/paramiko-call
```

## How to fix

1. Build the command from constants and `shlex.quote()` every variable part.
2. Or send the arguments over a channel that does not go through a shell.


## Examples

### Python

#### Don't

```python
import paramiko

paramiko.exec_command("ls " + path)
```

#### Do

```python
import paramiko

client = paramiko.SSHClient()
client.exec_command("uptime")
```

## Differences from the source

This is Ruff's check as written: a call whose name resolves to `paramiko.exec_command`. The usual `client.exec_command(…)` on an `SSHClient` instance does not resolve to that name and is not reported; Ruff does not report it either.
