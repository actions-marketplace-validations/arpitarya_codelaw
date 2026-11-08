---
id: js/no-missing-local-resource
title: "No `new URL(…, import.meta.url)` to a file that isn't there"
languages: [javascript, jsx, typescript, tsx]
fix: auto
fix_reason: "the subset: rewritten when the file exists in another case; a missing file is reported without a fix"
stability: preview
ported_from: [eslint-plugin-unicorn/no-missing-local-resource]
since: 0.5.0
---

`new URL("./file", import.meta.url)` with a relative string names a file next to the module. When no file (or, for a trailing `/`, no folder) is there, it is reported; when one is there in another case, the fix writes its case. Absolute and root-relative paths, URLs with a scheme and fragments are left alone. Upstream checks Markdown, HTML and CSS links as well; those are other packs' rules.

**Why:** The URL is resolved at run time, so a wrong path fails only when the code runs — and a case mismatch only on a case-sensitive file system.

```codelaw
native: js/no-missing-local-resource
```

## Examples

### JavaScript

#### Don't

```js files
// src/load.js
const url = new URL("./Data.json", import.meta.url);
// src/data.json
{}
```

#### Do

```js files
// src/load.js
const url = new URL("./data.json", import.meta.url);
// src/data.json
{}
```

#### Fixed

```js files
// src/load.js
const url = new URL("./data.json", import.meta.url);
// src/data.json
{}
```


### JSX

#### Don't

```jsx files
// src/load.jsx
const url = new URL("./Data.json", import.meta.url);
// src/data.json
{}
```

#### Do

```jsx files
// src/load.jsx
const url = new URL("./data.json", import.meta.url);
// src/data.json
{}
```

#### Fixed

```jsx files
// src/load.jsx
const url = new URL("./data.json", import.meta.url);
// src/data.json
{}
```


### TypeScript

#### Don't

```ts files
// src/load.ts
const url = new URL("./Data.json", import.meta.url);
// src/data.json
{}
```

#### Do

```ts files
// src/load.ts
const url = new URL("./data.json", import.meta.url);
// src/data.json
{}
```

#### Fixed

```ts files
// src/load.ts
const url = new URL("./data.json", import.meta.url);
// src/data.json
{}
```


### TSX

#### Don't

```tsx files
// src/load.tsx
const url = new URL("./Data.json", import.meta.url);
// src/data.json
{}
```

#### Do

```tsx files
// src/load.tsx
const url = new URL("./data.json", import.meta.url);
// src/data.json
{}
```

#### Fixed

```tsx files
// src/load.tsx
const url = new URL("./data.json", import.meta.url);
// src/data.json
{}
```
