---
id: js/no-useless-re-export
title: "No named re-export that `export *` from the same module already covers"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/no-useless-re-export]
since: 0.5.0
---

When a file has exactly one `export * from 'm'` (not `export * as ns`), an `export { x } from 'm'` of the same specifier, written the same way, re-exports `x` a second time. The specifier is reported and removed — the whole statement when every name in it is redundant. A renamed specifier (`x as y`), `default`, a type-only re-export, and a file with two or more different `export *` sources are left alone: the paths are compared as written, not resolved.

**Why:** The wildcard already exports the name; listing it again suggests it is special and keeps two places to change.

```codelaw
native: js/no-useless-re-export
```

## Examples

### JavaScript

#### Don't

```js
export * from "./util.js";
export { parse } from "./util.js";
```

#### Do

```js
export * from "./util.js";
export { parse as parseUtil } from "./util.js";
```

#### Fixed

```js
export * from "./util.js";
```


### JSX

#### Don't

```jsx
export * from "./util.js";
export { parse } from "./util.js";
```

#### Do

```jsx
export * from "./util.js";
export { parse as parseUtil } from "./util.js";
```

#### Fixed

```jsx
export * from "./util.js";
```


### TypeScript

#### Don't

```ts
export * from "./util.js";
export { parse } from "./util.js";
```

#### Do

```ts
export * from "./util.js";
export { parse as parseUtil } from "./util.js";
```

#### Fixed

```ts
export * from "./util.js";
```


### TSX

#### Don't

```tsx
export * from "./util.js";
export { parse } from "./util.js";
```

#### Do

```tsx
export * from "./util.js";
export { parse as parseUtil } from "./util.js";
```

#### Fixed

```tsx
export * from "./util.js";
```
