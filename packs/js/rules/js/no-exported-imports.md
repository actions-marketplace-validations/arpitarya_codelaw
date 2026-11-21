---
id: js/no-exported-imports
title: "No import that is exported in a separate statement"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the import and the export statement merge into one `export … from`, and the local binding goes"
fix_exception: restructure
stability: preview
ported_from: [biome/style/noExportedImports]
since: 0.5.1
---

A name brought in by `import` and then listed in `export { … }` (no `from`) is reported at its specifier, whether it is a named, a default or a namespace import. `export { A } from "mod"` is the form to write. Behaviour of Biome's `noExportedImports`.

**Why:** `export … from` says in one statement that the module only passes the name on, and it does not add a local binding that nothing reads.

```codelaw
native: js/no-exported-imports
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JavaScript

#### Don't

```js
import { parse } from "./parse";
export { parse };
```

#### Do

```js
export { parse } from "./parse";
```

### JSX

#### Don't

```jsx
import { parse } from "./parse";
export { parse };
```

#### Do

```jsx
export { parse } from "./parse";
```

### TypeScript

#### Don't

```ts
import { parse } from "./parse";
export { parse };
```

#### Do

```ts
export { parse } from "./parse";
```

### TSX

#### Don't

```tsx
import { parse } from "./parse";
export { parse };
```

#### Do

```tsx
export { parse } from "./parse";
```
