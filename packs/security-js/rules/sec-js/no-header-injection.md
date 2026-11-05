---
id: sec-js/no-header-injection
title: "No response header value taken from a request"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "encoding or stripping line breaks keeps the value out of the header structure, and the reviewer picks the one the header tolerates"
stability: preview
security: { cwe: [113], owasp: ["A05:2025"], severity: high }
since: 0.3.0
---

A response header is not set from request data unchanged.

**Why:** A value with a carriage return and a line feed in it ends the header and starts another one, or the body: response splitting (CWE-113).

```codelaw
match:
  any:
    - pattern: res.setHeader($N, $V)
    - pattern: res.header($N, $V)
    - pattern: res.set($N, $V)
    - pattern: res.append($N, $V)
    - pattern: response.setHeader($N, $V)
    - pattern: response.header($N, $V)
    - pattern: reply.header($N, $V)
    - pattern: ctx.set($N, $V)
    - pattern: ctx.append($N, $V)
    - pattern: h.header($N, $V)
where:
  $V: { from_source: { tables: [sources/express, sources/koa, sources/nextjs] } }
message: "`$V` comes from the request and is written into a response header"
fix:
  choices:
    - label: encode
      fix: {replace: {target: $V, with: "encodeURIComponent(String($V))"}}
    - label: strip line breaks
      fix: {replace: {target: $V, with: 'String($V).replace(/[\r\n]/g, "")'}}
```

## How to fix

1. Encode the value for a header (`encodeURIComponent`) or check it against an allow-list before it is written.
2. Reject a value that contains `\r` or `\n`.

## Examples

### JavaScript

#### Don't

```js
res.setHeader("X-Name", req.query.name);
```

#### Fixed (encode)

```js
res.setHeader("X-Name", encodeURIComponent(String(req.query.name)));
```

#### Fixed (strip line breaks)

```js
res.setHeader("X-Name", String(req.query.name).replace(/[\r\n]/g, ""));
```

#### Do

```js
res.setHeader("X-Name", encodeURIComponent(req.query.name));
```

### JSX

#### Don't

```jsx
res.setHeader("X-Name", req.query.name);
```

#### Fixed (encode)

```jsx
res.setHeader("X-Name", encodeURIComponent(String(req.query.name)));
```

#### Fixed (strip line breaks)

```jsx
res.setHeader("X-Name", String(req.query.name).replace(/[\r\n]/g, ""));
```

#### Do

```jsx
res.setHeader("X-Name", encodeURIComponent(req.query.name));
```

### TypeScript

#### Don't

```ts
res.setHeader("X-Name", req.query.name);
```

#### Fixed (encode)

```ts
res.setHeader("X-Name", encodeURIComponent(String(req.query.name)));
```

#### Fixed (strip line breaks)

```ts
res.setHeader("X-Name", String(req.query.name).replace(/[\r\n]/g, ""));
```

#### Do

```ts
res.setHeader("X-Name", encodeURIComponent(req.query.name));
```

### TSX

#### Don't

```tsx
res.setHeader("X-Name", req.query.name);
```

#### Fixed (encode)

```tsx
res.setHeader("X-Name", encodeURIComponent(String(req.query.name)));
```

#### Fixed (strip line breaks)

```tsx
res.setHeader("X-Name", String(req.query.name).replace(/[\r\n]/g, ""));
```

#### Do

```tsx
res.setHeader("X-Name", encodeURIComponent(req.query.name));
```
