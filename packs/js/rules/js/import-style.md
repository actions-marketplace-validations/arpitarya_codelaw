---
id: js/import-style
title: "Each module imported in its one style"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "changing an import's style changes every use of the names it binds"
fix_exception: restructure
options: { styles: { type: "object", default: {}, description: "module name to { default, named, namespace, unassigned } booleans, or false to stop checking it" }, extendDefaultStyles: { type: "boolean", default: true, description: "add styles to the built-in ones (path and chalk default, util named) instead of replacing them" }, checkImport: { type: "boolean", default: true, description: "check import declarations" }, checkDynamicImport: { type: "boolean", default: true, description: "check `await import()` assigned to a variable" }, checkRequire: { type: "boolean", default: true, description: "check `require()`" }, checkExportFrom: { type: "boolean", default: false, description: "check `export … from`" } }
stability: preview
ported_from: [eslint-plugin-unicorn/import-style]
since: 0.5.0
---

By default `path` and `chalk` (with or without `node:`) are imported with a default import and `util` with named imports; the `styles` option sets the style — `default`, `named`, `namespace`, `unassigned` — for any module. An `import`, `require()` (where `default` allows a whole-module binding too) or `await import()` in another style is reported. Type-only imports are not judged.

**Why:** One style per module keeps names consistent across a codebase.

```codelaw
native: js/import-style
```

## How to fix

1. Rewrite the import in the allowed style and its uses with it.

## Examples

### JavaScript

#### Don't

```js
import { join } from "node:path";
```

#### Do

```js
import path from "node:path";
```


### JSX

#### Don't

```jsx
import { join } from "node:path";
```

#### Do

```jsx
import path from "node:path";
```


### TypeScript

#### Don't

```ts
import { join } from "node:path";
```

#### Do

```ts
import path from "node:path";
```


### TSX

#### Don't

```tsx
import { join } from "node:path";
```

#### Do

```tsx
import path from "node:path";
```
