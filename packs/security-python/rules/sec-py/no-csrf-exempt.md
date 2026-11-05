---
id: sec-py/no-csrf-exempt
title: 'No `@csrf_exempt`'
languages: [python]
fix: suggest
fix_reason: "removing the decorator makes the view check CSRF, so a webhook stops working until it has its own authentication; the reviewer confirms the view is not one, and removes the import if nothing else uses it"
stability: preview
security: { cwe: [352], owasp: ["A01:2025"], severity: high }
since: 0.2.0
---

A view is not decorated with `@csrf_exempt`.

**Why:** Without the token check any page the user visits can submit the form as the user (CWE-352).

```codelaw
match:
  kind: decorator
  regex: '^@\s*((django\.views\.decorators\.csrf\.)|(csrf\.))?csrf_exempt\b'
message: 'This view skips the CSRF check'
fix:
  - remove: $$MATCH
```

## How to fix

1. Remove `@csrf_exempt` and send the token (`{% csrf_token %}`, or the `X-CSRFToken` header).
2. For a webhook, authenticate the sender (a signed header) and keep the exemption on that one view with a comment saying why.

## Examples

### Python

#### Don't

```py
from django.views.decorators.csrf import csrf_exempt

@csrf_exempt
def update(request):
    ...
```

#### Fixed

```py
from django.views.decorators.csrf import csrf_exempt

def update(request):
    ...
```

#### Do

```py
def update(request):
    ...
```
