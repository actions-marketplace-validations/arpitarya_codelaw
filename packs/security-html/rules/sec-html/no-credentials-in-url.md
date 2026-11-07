---
id: sec-html/no-credentials-in-url
title: "No username and password in a URL"
languages: [html]
fix: suggest
fix_reason: "the `user:password@` part of the URL is removed; the link then asks for credentials instead of carrying them, which the reviewer confirms"
stability: preview
security: { cwe: [522], owasp: ["A07:2025"], severity: high }
since: 0.2.0
---

A URL attribute does not carry `user:password@` in front of its host.

**Why:** A URL is stored in history, logs, referrers and bookmarks, so a credential written into one is disclosed to all of them.

```codelaw
native: sec-html/no-credentials-in-url
```

## How to fix

1. Remove the credentials from the URL.
2. Send them in an `Authorization` header, or keep the request on the server.

## Examples

### HTML

#### Don't

```html
<a href="https://admin:hunter2@intranet.example.com/">Admin</a>
```

#### Do

```html
<a href="https://intranet.example.com/">Admin</a>
```
