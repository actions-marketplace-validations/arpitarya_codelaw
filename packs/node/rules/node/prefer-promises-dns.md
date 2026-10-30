---
id: node/prefer-promises-dns
title: "Use the promise API of dns"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "a callback becomes an awaited promise, so the caller has to become async and handle errors with try/catch"
stability: preview
ported_from: [eslint-plugin-n/prefer-promises/dns]
since: 0.5.0
---

Code calls `dns.promises` (or `dns/promises`), not the callback form of a `dns` function. Reported: a call through the module object (`dns.readFile(...)`); a function destructured from the module is not followed.

**Why:** Callback APIs compose badly with async code: errors escape `try`/`catch`, and sequencing needs nesting.

```codelaw
match:
  kind: call_expression
  has:
    field: function
    kind: member_expression
    all:
      - has: { field: object, kind: identifier, pattern: $MOD }
      - has: { field: property, regex: ^(lookup|lookupService|resolve|resolve4|resolve6|resolveAny|resolveCaa|resolveCname|resolveMx|resolveNaptr|resolveNs|resolvePtr|resolveSoa|resolveSrv|resolveTxt|reverse)$ }
where:
  $MOD: { imported-from: [dns, "node:dns"] }
message: "Use the promise form of `dns` (`dns/promises`)"
```

## How to fix

1. Import the function from `dns/promises` (or use `dns.promises`) and `await` it.
2. Move the callback body after the `await`, and its error branch into a `catch`.

## Examples

### JavaScript

#### Don't

```js
import dns from "node:dns";
dns.lookup("example.com", (err, address) => {
  use(address);
});
```

#### Do

```js
import dns from "node:dns";
const { address } = await dns.promises.lookup("example.com");
use(address);
```

### JSX

#### Don't

```jsx
import dns from "node:dns";
dns.lookup("example.com", (err, address) => {
  use(address);
});
```

#### Do

```jsx
import dns from "node:dns";
const { address } = await dns.promises.lookup("example.com");
use(address);
```

### TypeScript

#### Don't

```ts
import dns from "node:dns";
dns.lookup("example.com", (err, address) => {
  use(address);
});
```

#### Do

```ts
import dns from "node:dns";
const { address } = await dns.promises.lookup("example.com");
use(address);
```

### TSX

#### Don't

```tsx
import dns from "node:dns";
dns.lookup("example.com", (err, address) => {
  use(address);
});
```

#### Do

```tsx
import dns from "node:dns";
const { address } = await dns.promises.lookup("example.com");
use(address);
```
