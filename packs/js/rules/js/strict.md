---
id: js/strict
title: "No \"use strict\" in a module"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint/strict]
since: 0.5.0
---

A file with `import` or `export` (a module) does not contain a `"use strict"` directive. Scripts are left alone.

**Why:** A module is strict by definition, so the directive is noise.

```codelaw

match:
  kind: expression_statement
  has: { kind: string, regex: '^.use strict.$' }
  inside: { kind: program, has: { any: [{kind: import_statement}, {kind: export_statement}] } }
message: "Modules are already strict; remove the directive"
fix:
  - remove
```

## Examples

### JavaScript

#### Don't

```js
"use strict";
export const x = 1;
```

#### Do

```js
export const x = 1;
```

### JSX

#### Don't

```jsx
"use strict";
export const x = 1;
```

#### Do

```jsx
export const x = 1;
```

### TypeScript

#### Don't

```ts
"use strict";
export const x = 1;
```

#### Do

```ts
export const x = 1;
```

### TSX

#### Don't

```tsx
"use strict";
export const x = 1;
```

#### Do

```tsx
export const x = 1;
```
