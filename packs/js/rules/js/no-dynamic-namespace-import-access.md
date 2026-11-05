---
id: js/no-dynamic-namespace-import-access
title: "No computed access on a namespace import"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "naming the exports used means the author choosing them"
fix_exception: restructure
stability: preview
ported_from: [biome/performance/noDynamicNamespaceImportAccess]
since: 0.5.0
---

A namespace import (`import * as ns`) is not read with a computed key (`ns[key]`).

**Why:** A computed access makes the bundler keep every export of the module, defeating tree shaking.

```codelaw
match:
  kind: subscript_expression
  has: { field: object, kind: identifier, pattern: $NS }
  inside:
    kind: program
    stopBy: end
    has: { pattern: "import * as $NS from $SRC" }
message: "`$NS[…]` keeps every export of the module; import the names you use"
```

## How to fix

1. Import the names you need and pick between them explicitly (a `switch` or an object of the imports).

## Examples

### JavaScript

#### Don't

```js
import * as icons from './icons';
export const icon = (name) => icons[name];
```

#### Do

```js
import { add, remove } from './icons';
const icons = { add, remove };
export const icon = (name) => icons[name];
```

### JSX

#### Don't

```jsx
import * as icons from './icons';
export const icon = (name) => icons[name];
```

#### Do

```jsx
import { add, remove } from './icons';
const icons = { add, remove };
export const icon = (name) => icons[name];
```

### TypeScript

#### Don't

```ts
import * as icons from './icons';
export const icon = (name) => icons[name];
```

#### Do

```ts
import { add, remove } from './icons';
const icons = { add, remove };
export const icon = (name) => icons[name];
```

### TSX

#### Don't

```tsx
import * as icons from './icons';
export const icon = (name) => icons[name];
```

#### Do

```tsx
import { add, remove } from './icons';
const icons = { add, remove };
export const icon = (name) => icons[name];
```
