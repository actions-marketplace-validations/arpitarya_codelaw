---
id: js/prefer-export-from
title: "`export … from` for an import that is only re-exported"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-export-from]
since: 0.5.0
---

An imported binding exported again — `export default x`, `export { x as y }`, or `export const y = x` (no annotation, `y` unused, no doc comment) — is reported at the export and written as `export { x as y } from "…"` at the end of the file (`export * as ns from` for a namespace). The import goes when the export was its only use; otherwise it stays for the code that uses it. A namespace exported as `default`, and a comment in either statement, leave it without a fix.

**Why:** A re-export written as one statement says what it is, and the module needs no local binding for it.

```codelaw
native: js/prefer-export-from
```

## Examples

### JavaScript

#### Don't

```js
import { parse } from "./parse.js";
export { parse };
```

#### Do

```js
export { parse } from "./parse.js";
```

#### Fixed

```js
export { parse } from "./parse.js";
```


### JSX

#### Don't

```jsx
import { parse } from "./parse.js";
export { parse };
```

#### Do

```jsx
export { parse } from "./parse.js";
```

#### Fixed

```jsx
export { parse } from "./parse.js";
```


### TypeScript

#### Don't

```ts
import { parse } from "./parse.js";
export { parse };
```

#### Do

```ts
export { parse } from "./parse.js";
```

#### Fixed

```ts
export { parse } from "./parse.js";
```


### TSX

#### Don't

```tsx
import { parse } from "./parse.js";
export { parse };
```

#### Do

```tsx
export { parse } from "./parse.js";
```

#### Fixed

```tsx
export { parse } from "./parse.js";
```
