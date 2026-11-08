---
id: js/import-export-star
title: "No name brought in by two export-stars"
languages: [javascript, jsx, typescript, tsx]
scope: project
fix: manual
fix_reason: "which module's export is meant is the author's knowledge, and settling it renames or drops an export across files"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-import/export, eslint-plugin-import-x/export]
since: 0.5.0
---

The half of `import/export` that needs other modules: a name that two `export * from` statements of one module both bring in, or that an `export *` and a named export of the module both do, is reported at each statement involved (`default` is not passed on by `export *`). An `export * from` of a module with no named export is reported too. A type and a value of one name are two exports. One module's own duplicate exports are `js/import-export`'s.

**Why:** the language drops the name from the module's exports when two stars supply it, so importers get a link error or `undefined` instead of either.

```codelaw
graph:
  export:
    star: conflicts
message: "More than one export of `$NAME` reaches this module's exports"
```

## How to fix

1. Export the one you mean by name: `export { x } from "./a"`, which wins over the stars.
2. Or rename one of the exports at its source.

## Examples

### JavaScript

#### Don't

```js files
// src/a.js
export const x = 1;
// src/b.js
export const x = 2;
// src/index.js
export * from "./a";
export * from "./b";
```

#### Do

```js files
// src/a.js
export const x = 1;
// src/b.js
export const y = 2;
// src/index.js
export * from "./a";
export * from "./b";
```

### JSX

#### Don't

```jsx files
// src/a.jsx
export const x = 1;
// src/b.jsx
export const x = 2;
// src/index.jsx
export * from "./a";
export * from "./b";
```

#### Do

```jsx files
// src/a.jsx
export const x = 1;
// src/b.jsx
export const y = 2;
// src/index.jsx
export * from "./a";
export * from "./b";
```

### TypeScript

#### Don't

```ts files
// src/a.ts
export const x = 1;
// src/b.ts
export const x = 2;
// src/index.ts
export * from "./a";
export * from "./b";
```

#### Do

```ts files
// src/a.ts
export const x = 1;
// src/b.ts
export const y = 2;
// src/index.ts
export * from "./a";
export * from "./b";
```

### TSX

#### Don't

```tsx files
// src/a.tsx
export const x = 1;
// src/b.tsx
export const x = 2;
// src/index.tsx
export * from "./a";
export * from "./b";
```

#### Do

```tsx files
// src/a.tsx
export const x = 1;
// src/b.tsx
export const y = 2;
// src/index.tsx
export * from "./a";
export * from "./b";
```
