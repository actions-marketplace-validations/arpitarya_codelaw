---
id: js/import-dynamic-import-chunkname
title: "Dynamic imports name their webpack chunk"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "the chunk name becomes the file name of the bundle, which the author may want shorter"
stability: preview
ported_from: [eslint-plugin-import/dynamic-import-chunkname]
since: 0.5.0
---

Each `import()` of a string path carries a leading `/* webpackChunkName: "…" */` comment. The fix names the chunk after the path; it is offered only for a path that is already a valid chunk name (letters, digits, `-`, `_`, `/`, `.`). An `import()` of a computed path is not reported.

**Why:** Without a name webpack numbers the chunk, so bundle reports, network panels and cache keys show `123.js` instead of what was loaded.

```codelaw
match:
  kind: call_expression
  has: { field: function, kind: import }
  all:
    - has:
        field: arguments
        has:
          kind: string
          pattern: $SRC
          has: { kind: string_fragment, regex: '^[0-9a-zA-Z_/.-]+$', pattern: $P }
    - not:
        has:
          field: arguments
          has: { kind: comment, regex: 'webpackChunkName' }
message: "`import(\"$P\")` has no `webpackChunkName` comment; name the chunk"
fix:
  - insert-before: { target: $SRC, text: "/* webpackChunkName: \"$P\" */ " }
```

## Examples

### JavaScript

#### Don't

```js
const page = import("./pages/settings");
use(page);
```

#### Do

```js
const page = import(/* webpackChunkName: "./pages/settings" */ "./pages/settings");
use(page);
```

### JSX

#### Don't

```jsx
const page = import("./pages/settings");
use(page);
```

#### Do

```jsx
const page = import(/* webpackChunkName: "./pages/settings" */ "./pages/settings");
use(page);
```

### TypeScript

#### Don't

```ts
const page = import("./pages/settings");
use(page);
```

#### Do

```ts
const page = import(/* webpackChunkName: "./pages/settings" */ "./pages/settings");
use(page);
```

### TSX

#### Don't

```tsx
const page = import("./pages/settings");
use(page);
```

#### Do

```tsx
const page = import(/* webpackChunkName: "./pages/settings" */ "./pages/settings");
use(page);
```
