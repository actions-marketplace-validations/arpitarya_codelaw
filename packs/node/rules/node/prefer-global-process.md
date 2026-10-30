---
id: node/prefer-global-process
title: "Use the global process"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "a bundler for the browser may resolve `process` to a polyfill package rather than Node's built-in, which the global is not"
stability: preview
ported_from: [eslint-plugin-n/prefer-global/process]
since: 0.5.0
---

Code uses the global `process` rather than importing it from `process` (the upstream default, `"always"`). Reported: a `require` or `import` that binds only that name, which the fix removes; other shapes are left alone.

**Why:** The global is the same object as the module's export; the import is one more line that says nothing and invites the two to be confused.

```codelaw
match:
  any:
    - pattern: "const process = require($M)"
    - pattern: "let process = require($M)"
    - pattern: "var process = require($M)"
    - pattern: "import process from $M"
constraints:
  M: { regex: '^.(node:)?process.$' }
message: "`process` is a global; drop this import"
fix:
  - remove
```

## Examples

### JavaScript

#### Don't

```js
const process = require("process");
process.exitCode = 1;
```

#### Do

```js
process.exitCode = 1;
```

### JSX

#### Don't

```jsx
const process = require("process");
process.exitCode = 1;
```

#### Do

```jsx
process.exitCode = 1;
```

### TypeScript

#### Don't

```ts
const process = require("process");
process.exitCode = 1;
```

#### Do

```ts
process.exitCode = 1;
```

### TSX

#### Don't

```tsx
const process = require("process");
process.exitCode = 1;
```

#### Do

```tsx
process.exitCode = 1;
```
