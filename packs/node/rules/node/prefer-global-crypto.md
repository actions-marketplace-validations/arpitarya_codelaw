---
id: node/prefer-global-crypto
title: "Use the global crypto"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "a bundler for the browser may resolve `crypto` to a polyfill package rather than Node's built-in, which the global is not"
stability: preview
ported_from: [eslint-plugin-n/prefer-global/crypto]
since: 0.5.0
---

Code uses the global `crypto` rather than importing it from `crypto` (the upstream default, `"always"`). Reported: a `require` or `import` that binds only that name, which the fix removes; other shapes are left alone.

**Why:** The global is the same object as the module's export; the import is one more line that says nothing and invites the two to be confused.

```codelaw
match:
  any:
    - pattern: "const crypto = require($M).webcrypto"
    - pattern: "let crypto = require($M).webcrypto"
    - pattern: "var crypto = require($M).webcrypto"
    - pattern: "const { webcrypto: crypto } = require($M)"
    - pattern: "import { webcrypto as crypto } from $M"
constraints:
  M: { regex: '^.(node:)?crypto.$' }
message: "`crypto` is a global; drop this import"
fix:
  - remove
```

## Examples

### JavaScript

#### Don't

```js
const { webcrypto: crypto } = require("crypto");
const id = crypto.randomUUID();
```

#### Do

```js
const id = crypto.randomUUID();
```

### JSX

#### Don't

```jsx
const { webcrypto: crypto } = require("crypto");
const id = crypto.randomUUID();
```

#### Do

```jsx
const id = crypto.randomUUID();
```

### TypeScript

#### Don't

```ts
const { webcrypto: crypto } = require("crypto");
const id = crypto.randomUUID();
```

#### Do

```ts
const id = crypto.randomUUID();
```

### TSX

#### Don't

```tsx
const { webcrypto: crypto } = require("crypto");
const id = crypto.randomUUID();
```

#### Do

```tsx
const id = crypto.randomUUID();
```
