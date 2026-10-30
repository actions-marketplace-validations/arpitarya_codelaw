---
id: node/prefer-global-console
title: "Use the global console"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "a bundler for the browser may resolve `console` to a polyfill package rather than Node's built-in, which the global is not"
stability: preview
ported_from: [eslint-plugin-n/prefer-global/console]
since: 0.5.0
---

Code uses the global `console` rather than importing it from `console` (the upstream default, `"always"`). Reported: a `require` or `import` that binds only that name, which the fix removes; other shapes are left alone.

**Why:** The global is the same object as the module's export; the import is one more line that says nothing and invites the two to be confused.

```codelaw
match:
  any:
    - pattern: "const console = require($M)"
    - pattern: "let console = require($M)"
    - pattern: "var console = require($M)"
    - pattern: "import console from $M"
constraints:
  M: { regex: '^.(node:)?console.$' }
message: "`console` is a global; drop this import"
fix:
  - remove
```

## Examples

### JavaScript

#### Don't

```js
const console = require("console");
console.log("ready");
```

#### Do

```js
console.log("ready");
```

### JSX

#### Don't

```jsx
const console = require("console");
console.log("ready");
```

#### Do

```jsx
console.log("ready");
```

### TypeScript

#### Don't

```ts
const console = require("console");
console.log("ready");
```

#### Do

```ts
console.log("ready");
```

### TSX

#### Don't

```tsx
const console = require("console");
console.log("ready");
```

#### Do

```tsx
console.log("ready");
```
