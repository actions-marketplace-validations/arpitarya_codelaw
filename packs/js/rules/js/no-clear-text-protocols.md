---
id: js/no-clear-text-protocols
title: "No http:// or ftp:// URLs"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "whether the host supports TLS must be checked"
stability: preview
ported_from: [eslint-plugin-sonarjs/no-clear-text-protocols]
since: 0.5.0
---

A string does not hold an `http://`, `ftp://` or `telnet://` URL to a host other than localhost (XML namespaces on w3.org are left alone).

**Why:** Clear-text protocols can be read and changed by anyone on the network path.

```codelaw
match:
  kind: string_fragment
  regex: ^(http|ftp|telnet)://
  not:
    regex: ^\w+://(localhost|127\.0\.0\.1|\[::1\]|0\.0\.0\.0)([:/]|$)
  all:
  - not:
      regex: ^http://(www\.)?w3\.org/
message: This URL is sent in clear text; use https:// (or sftp, ssh)
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JavaScript

#### Don't

```js
const api = "http://api.example.com";
```

#### Do

```js
const api = "https://api.example.com";
```

### JSX

#### Don't

```jsx
const api = "http://api.example.com";
```

#### Do

```jsx
const api = "https://api.example.com";
```

### TypeScript

#### Don't

```ts
const api = "http://api.example.com";
```

#### Do

```ts
const api = "https://api.example.com";
```

### TSX

#### Don't

```tsx
const api = "http://api.example.com";
```

#### Do

```tsx
const api = "https://api.example.com";
```
