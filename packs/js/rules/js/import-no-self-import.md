---
id: js/import-no-self-import
title: "No module that imports itself"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the import binds names the module uses, so removing it means rewriting those uses"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-import/no-self-import]
since: 0.5.0
---

An `import`, `export … from`, `import()` or `require()` of a relative path that resolves (with an extension or `index` added) to the file itself is reported.

**Why:** A module that imports itself gets its own half-initialised exports, a cycle in one file.

```codelaw
native: js/import-no-self-import
```

## How to fix

1. Use the local names directly and remove the import.

## Examples

### JavaScript

#### Don't

```js files
// src/foo.js
import { a } from "./foo";
export const b = a;
```

#### Do

```js files
// src/foo.js
import { a } from "./bar";
export const b = a;
// src/bar.js
export const a = 1;
```


### JSX

#### Don't

```jsx files
// src/foo.jsx
import { a } from "./foo";
export const b = a;
```

#### Do

```jsx files
// src/foo.jsx
import { a } from "./bar";
export const b = a;
// src/bar.jsx
export const a = 1;
```


### TypeScript

#### Don't

```ts files
// src/foo.ts
import { a } from "./foo";
export const b = a;
```

#### Do

```ts files
// src/foo.ts
import { a } from "./bar";
export const b = a;
// src/bar.ts
export const a = 1;
```


### TSX

#### Don't

```tsx files
// src/foo.tsx
import { a } from "./foo";
export const b = a;
```

#### Do

```tsx files
// src/foo.tsx
import { a } from "./bar";
export const b = a;
// src/bar.tsx
export const a = 1;
```
