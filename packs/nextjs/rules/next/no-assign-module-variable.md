---
id: next/no-assign-module-variable
title: "No variable named module"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "the variable is renamed to the first of `mod`, `mod2`, … the file does not use, with every reference; the name is the author's to improve, and an exported one is renamed by hand"
stability: preview
ported_from: ["@next/eslint-plugin-next/no-assign-module-variable"]
since: 0.5.0
---

No variable is declared with the name `module`.

**Why:** In a Next.js page it shadows the module object the bundler relies on, which breaks the build in confusing ways.

```codelaw
match:
  kind: variable_declarator
  has: { field: name, kind: identifier, regex: '^module$', pattern: $N }
message: "Do not declare a variable named `module`"
fix:
  - rename-symbol: { target: $N, to: "$FRESH(mod)", scope: binding }
```

## How to fix

1. Rename the variable (`mod`, `entry`, …).

## Examples

### JavaScript

#### Don't

```js
let module = await load();
```

#### Do

```js
let mod = await load();
```

### JSX

#### Don't

```jsx
let module = await load();
```

#### Do

```jsx
let mod = await load();
```

### TypeScript

#### Don't

```ts
let module = await load();
```

#### Do

```ts
let mod = await load();
```

### TSX

#### Don't

```tsx
let module = await load();
```

#### Do

```tsx
let mod = await load();
```
