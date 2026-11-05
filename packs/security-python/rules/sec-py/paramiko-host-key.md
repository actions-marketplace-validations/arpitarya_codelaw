---
id: sec-py/paramiko-host-key
title: 'SSH host keys are checked'
languages: [python]
fix: suggest
fix_reason: "`RejectPolicy` refuses an unknown host, which breaks a connection that relied on accepting it; where the known keys come from (`load_system_host_keys`, a known_hosts file) is deployment-specific"
stability: preview
ported_from: [bandit/B507, ruff/S507]
security: { cwe: [295], owasp: ["A07:2025"], severity: high }
since: 0.2.0
---

`paramiko` is not told to accept unknown host keys with `AutoAddPolicy` or `WarningPolicy`.

**Why:** A connection to an impostor goes through unnoticed, and the credentials are handed to it (CWE-295).

```codelaw
match: { any: [{ pattern: '$C.set_missing_host_key_policy($P)' }] }
constraints: { P: { regex: '(AutoAddPolicy|WarningPolicy)' } }
message: 'This accepts any SSH host key; load known hosts and reject unknown ones'
fix:
  - replace: { target: $P, with: "paramiko.RejectPolicy()" }
```

## Examples

### Python

#### Don't

```py
client.set_missing_host_key_policy(paramiko.AutoAddPolicy())
```

#### Fixed

```py
client.set_missing_host_key_policy(paramiko.RejectPolicy())
```

#### Do

```py
client.load_system_host_keys()
client.set_missing_host_key_policy(paramiko.RejectPolicy())
```
