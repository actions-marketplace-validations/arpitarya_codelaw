---
id: sec-js/no-stack-trace-response
title: "No error object or stack trace in a response"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "the client gets a generic message and the error is logged first, and the reviewer confirms the client needs nothing else (a JSON client may want an object)"
stability: preview
security: { cwe: [209], owasp: ["A10:2025"], severity: medium }
since: 0.3.0
---

A response is not given an error object or its `stack`.

**Why:** A stack trace names files, versions, queries and sometimes values, which is the map an attacker uses for the next step (CWE-209).

```codelaw
match:
  any:
    - all:
        - pattern: $R.send($E)
        - regex: '^(res|response|reply)\b'
    - all:
        - pattern: $R.json($E)
        - regex: '^(res|response|reply)\b'
    - all:
        - pattern: $R.end($E)
        - regex: '^(res|response|reply)\b'
    - all:
        - pattern: $R.status($S).send($E)
    - all:
        - pattern: $R.status($S).json($E)
constraints:
  E: { regex: '^(err|error|e|ex|exception)$|\.stack\b|:\s*(err|error|ex|exception)\s*[,}]|\.stack\s*[,}]' }
message: "An error or its stack is sent to the client"
fix:
  - insert-before: {target: $$MATCH, text: "(console.error($E), "}
  - replace: {target: $E, with: '"Internal Server Error"'}
  - insert-after: {target: $$MATCH, text: ")"}
```

## How to fix

1. Log the error on the server (`logger.error(err)`), and send the client a generic message and, if useful, a request id to quote.
2. Set `NODE_ENV=production` so Express's default error handler hides the stack.

## Examples

### JavaScript

#### Don't

```js
res.status(500).send(err.stack);
```

#### Fixed

```js
(console.error(err.stack), res.status(500).send("Internal Server Error"));
```

#### Do

```js
logger.error(err);
res.status(500).send("Something went wrong");
```

### JSX

#### Don't

```jsx
res.status(500).send(err.stack);
```

#### Fixed

```jsx
(console.error(err.stack), res.status(500).send("Internal Server Error"));
```

#### Do

```jsx
logger.error(err);
res.status(500).send("Something went wrong");
```

### TypeScript

#### Don't

```ts
res.status(500).send(err.stack);
```

#### Fixed

```ts
(console.error(err.stack), res.status(500).send("Internal Server Error"));
```

#### Do

```ts
logger.error(err);
res.status(500).send("Something went wrong");
```

### TSX

#### Don't

```tsx
res.status(500).send(err.stack);
```

#### Fixed

```tsx
(console.error(err.stack), res.status(500).send("Internal Server Error"));
```

#### Do

```tsx
logger.error(err);
res.status(500).send("Something went wrong");
```
