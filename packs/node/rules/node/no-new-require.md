---
id: node/no-new-require
title: "No new require(...)"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "`new require(x)` and `new (require(x))()` call different functions; the rewrite is what was meant, not what runs now"
stability: preview
ported_from: [eslint-plugin-n/no-new-require]
since: 0.5.0
---

`new` is applied to what a module exports, not to `require` itself.

**Why:** `new require('x')` constructs `require` with `'x'` as its argument; it only appears to work, and reads as constructing the module.

```codelaw
match:
  kind: new_expression
  pattern: new $R($$$A)
constraints:
  R: { regex: ^require$ }
where:
  $R: undeclared
message: "Require the module first, then construct it: `new (require($$$A))()`"
fix: "new (require($$$A))()"
```

## Examples

### JavaScript

#### Don't

```js
const server = new require("./server")(options);
```

#### Do

```js
const server = new (require("./server"))()(options);
```

### JSX

#### Don't

```jsx
const server = new require("./server")(options);
```

#### Do

```jsx
const server = new (require("./server"))()(options);
```

### TypeScript

#### Don't

```ts
const server = new require("./server")(options);
```

#### Do

```ts
const server = new (require("./server"))()(options);
```

### TSX

#### Don't

```tsx
const server = new require("./server")(options);
```

#### Do

```tsx
const server = new (require("./server"))()(options);
```
