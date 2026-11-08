---
id: js/file-name-differ-from-class
title: "A file that exports only a default is named after it"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "renaming the file, or the export, changes what other files import"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-sonarjs/file-name-differ-from-class]
since: 0.5.0
---

A file with a default export and no named exports, whose default is a named class or function or a name bound to one (or to a `const`), has that name: the file name (or the name before its last extra dot, `Widget.dev.js`) equals the export's once `_`, `-` and `.` are dropped and case is ignored. `index` files are left alone. It is reported at the start of the file.

**Why:** When the file is named after what it exports, an import and the file it comes from are easy to match.

```codelaw
native: js/file-name-differ-from-class
```

## How to fix

1. Rename the file to the export's name, or rename the export.

## Examples

### JavaScript

#### Don't

```js path=src/widget.js
export default class Gadget {}
```

#### Do

```js path=src/gadget.js
export default class Gadget {}
```


### JSX

#### Don't

```jsx path=src/widget.jsx
export default class Gadget {}
```

#### Do

```jsx path=src/gadget.jsx
export default class Gadget {}
```


### TypeScript

#### Don't

```ts path=src/widget.ts
export default class Gadget {}
```

#### Do

```ts path=src/gadget.ts
export default class Gadget {}
```


### TSX

#### Don't

```tsx path=src/widget.tsx
export default class Gadget {}
```

#### Do

```tsx path=src/gadget.tsx
export default class Gadget {}
```
