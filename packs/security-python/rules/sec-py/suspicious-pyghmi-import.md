---
id: sec-py/suspicious-pyghmi-import
title: "`pyghmi` is imported"
languages: [python]
fix: manual
fix_reason: "the protocol is the problem, and the replacement is a different protocol"
fix_exception: security
stability: preview
ported_from: [ruff/S415]
security: { cwe: [319], owasp: ["A02:2025"], severity: medium }
since: 0.5.0
---

`import pyghmi` and `from pyghmi import …` are reported. `pyghmi` is an IPMI library.

**Why:** IPMI sends authentication material that can be captured and cracked offline, and many BMCs expose it by default. An encrypted management protocol (Redfish over TLS, SSH to the BMC) does not have the weakness (CWE-319).

```codelaw
native: sec-py/suspicious-pyghmi-import
```

## How to fix

1. Prefer a Redfish client over TLS for out-of-band management.
2. Where IPMI is unavoidable, keep it on an isolated management network and use IPMI 2.0 with strong, unique credentials.


## Examples

### Python

#### Don't

```python
import pyghmi

session = pyghmi.ipmi.command.Command(bmc=host, userid=user, password=password)
```

#### Do

```python
import requests

resp = requests.get(f"https://{host}/redfish/v1/", timeout=10)
```

## Differences from the source

Only the exact module `pyghmi` is matched, as in Ruff: `import pyghmi.ipmi` and `from pyghmi.ipmi import command` are not.
