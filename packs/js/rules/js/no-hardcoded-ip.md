---
id: js/no-hardcoded-ip
title: "No hard-coded IP address"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the configuration key is the author's choice"
stability: preview
ported_from: [eslint-plugin-sonarjs/no-hardcoded-ip]
since: 0.5.0
---

A string literal is not a public IPv4 address (loopback, private ranges and the broadcast address are left alone).

**Why:** An address in code cannot change without a release and leaks infrastructure details.

```codelaw

match:
  kind: string_fragment
  regex: '^(\w+://)?((25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)\.){3}(25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)(:\d+)?(/.*)?$'
  not: { regex: '^(\w+://)?(127\.|0\.0\.0\.0|255\.255\.255\.255|10\.|192\.168\.|2\.5\.)' }
message: "A hard-coded IP address; read it from configuration"
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JavaScript

#### Don't

```js
const api = "203.0.113.7";
```

#### Do

```js
const api = process.env.API_HOST;
```

### JSX

#### Don't

```jsx
const api = "203.0.113.7";
```

#### Do

```jsx
const api = process.env.API_HOST;
```

### TypeScript

#### Don't

```ts
const api = "203.0.113.7";
```

#### Do

```ts
const api = process.env.API_HOST;
```

### TSX

#### Don't

```tsx
const api = "203.0.113.7";
```

#### Do

```tsx
const api = process.env.API_HOST;
```
