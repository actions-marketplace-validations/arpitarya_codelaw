---
id: js/no-process-global
title: "Import `process` instead of using the global"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "the import is added after the last import of the file, and an author who relies on a bundler replacing `process.env` keeps the global"
stability: preview
ported_from: [biome/correctness/noProcessGlobal, deno_lint/no-process-global]
since: 0.5.1
---

`process` is not read as a global: it is imported first (`import process from "node:process"`). A file that declares or imports its own `process` is left alone. Behaviour of Biome's `noProcessGlobal`.

**Why:** The global ties a module to Node.js; the import says so, works the same where `process` is not a global (Deno, Bun, edge runtimes) and keeps the dependency visible.

```codelaw
native: js/no-process-global
```

## Examples

### JavaScript

#### Don't

```js
const mode = process.env.NODE_ENV;
```

#### Do

```js
import process from "node:process";
const mode = process.env.NODE_ENV;
```

### JSX

#### Don't

```jsx
const mode = process.env.NODE_ENV;
```

#### Do

```jsx
import process from "node:process";
const mode = process.env.NODE_ENV;
```

### TypeScript

#### Don't

```ts
const mode = process.env.NODE_ENV;
```

#### Do

```ts
import process from "node:process";
const mode = process.env.NODE_ENV;
```

### TSX

#### Don't

```tsx
const mode = process.env.NODE_ENV;
```

#### Do

```tsx
import process from "node:process";
const mode = process.env.NODE_ENV;
```
