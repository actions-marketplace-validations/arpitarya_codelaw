---
id: js/jsdoc-prefer-import-tag
title: "@import a type instead of inline import()"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the type comes in once by a new file-level `@import` block, whose place among the file's imports, and whose name against the names the file already binds, are the author's to decide"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-jsdoc/prefer-import-tag]
since: 0.5.0
---

A JSDoc type does not write `import("module").Name` inline; the type is brought in once with `@import { Name } from "module"` and used by name. `@typedef` tags are exempt, as upstream exempts them by default.

**Why:** Inline `import()` types repeat the module path at every use, are hard to read, and must each be edited when the module moves.

```codelaw
match:
  kind: comment
  all:
    - regex: '^/\*\*[^*/]'
    - regex: '(^/\*\*|\n)[ \t]*(\*[ \t]*)?@(param|arg|argument|returns?|type|property|prop|this|throws|yields|satisfies|template)\b[ \t]*\{[^}\n]*\bimport\('
message: "Bring the type in once with `@import { Name } from \"module\"` and use it by name"
```

## How to fix

1. Add a block `/** @import { Name } from "module" */` near the top of the file.
2. Replace each `import("module").Name` in the JSDoc types with `Name`.

## Examples

### JavaScript

#### Don't

```js
/**
 * Starts the app.
 * @param {import("./config").Config} config the settings
 */
function start(config) {}
```

#### Do

```js
/** @import { Config } from "./config" */

/**
 * Starts the app.
 * @param {Config} config the settings
 */
function start(config) {}
```

### JSX

#### Don't

```jsx
/**
 * Starts the app.
 * @param {import("./config").Config} config the settings
 */
function start(config) {}
```

#### Do

```jsx
/** @import { Config } from "./config" */

/**
 * Starts the app.
 * @param {Config} config the settings
 */
function start(config) {}
```

### TypeScript

#### Don't

```ts
/**
 * Starts the app.
 * @param {import("./config").Config} config the settings
 */
function start(config) {}
```

#### Do

```ts
/** @import { Config } from "./config" */

/**
 * Starts the app.
 * @param {Config} config the settings
 */
function start(config) {}
```

### TSX

#### Don't

```tsx
/**
 * Starts the app.
 * @param {import("./config").Config} config the settings
 */
function start(config) {}
```

#### Do

```tsx
/** @import { Config } from "./config" */

/**
 * Starts the app.
 * @param {Config} config the settings
 */
function start(config) {}
```
