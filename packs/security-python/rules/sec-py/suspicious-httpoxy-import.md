---
id: sec-py/suspicious-httpoxy-import
title: "`CGIHandler` or `CGIScript` is imported"
languages: [python]
fix: manual
fix_reason: "the replacement is a different server model, not an edit"
fix_exception: security
stability: preview
ported_from: [ruff/S412]
security: { cwe: [20], owasp: ["A05:2025"], severity: medium }
since: 0.5.0
---

`from wsgiref.handlers import CGIHandler` and `from twisted.web.twcgi import CGIScript` are reported at the module name.

**Why:** CGI passes the `Proxy` request header to the program as the `HTTP_PROXY` environment variable, which HTTP clients in the same process then obey: a request can route the program's outgoing calls through an attacker's proxy (httpoxy, CVE-2016-5385). Avoiding CGI for web applications avoids the class of attack.

```codelaw
native: sec-py/suspicious-httpoxy-import
```

## How to fix

1. Serve the application through a WSGI or ASGI server (`gunicorn`, `uvicorn`, `waitress`) instead of CGI.
2. If CGI cannot be avoided, drop `HTTP_PROXY` from the environment before any outgoing request.


## Examples

### Python

#### Don't

```python
from wsgiref.handlers import CGIHandler

CGIHandler().run(app)
```

#### Do

```python
from wsgiref.simple_server import make_server

make_server("", 8000, app).serve_forever()
```

## Differences from the source

Only the two `from … import` forms are reported, as in Ruff; `import wsgiref.handlers` and a later `wsgiref.handlers.CGIHandler` are not. The CWE is codelaw's mapping (20, input validation); Ruff does not give one.
