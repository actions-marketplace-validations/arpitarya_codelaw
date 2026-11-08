---
id: js/no-default-export
title: "No default exports"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "the default export of a named function, class or binding becomes a named export, and every project module's default import of it a named import, as one fix; a diff to accept, as code outside the project may import the default. An anonymous default, or a module re-exported or loaded by `require` or `import()`, offers no fix (SR-MULTI-FILE-FIXES)"
stability: preview
ported_from: [eslint-plugin-import/no-default-export]
since: 0.1.0
---

Modules export by name; `export default` is not used.

**Why:** A named export has one name everywhere, so it can be found, renamed and auto-imported
reliably.

```codelaw
native: js/no-default-export
```

## How to fix

1. Accept the fix: `codelaw fix --unsafe` exports the value by name and changes every `import x from` of this module to `import { name as x } from` (the diff names each file).
2. An anonymous default export has no name to give: name it, then run the fix again.

## Examples

### JavaScript

#### Don't

```js
export default function load() {}
```

#### Do

```js
export function load() {}
```

#### Don't

```js
// src/load.js
export default function load() {}
// src/app.js
import run from "./load";
run();
```

#### Do

```js
// src/load.js
export function load() {}
// src/app.js
import { load as run } from "./load";
run();
```

### JSX

#### Don't

```jsx
export default function load() {}
```

#### Do

```jsx
export function load() {}
```

#### Don't

```jsx
// src/load.jsx
export default function load() {}
// src/app.jsx
import run from "./load";
run();
```

#### Do

```jsx
// src/load.jsx
export function load() {}
// src/app.jsx
import { load as run } from "./load";
run();
```

### TypeScript

#### Don't

```ts
export default function load() {}
```

#### Do

```ts
export function load() {}
```

#### Don't

```ts
// src/load.ts
export default function load() {}
// src/app.ts
import run from "./load";
run();
```

#### Do

```ts
// src/load.ts
export function load() {}
// src/app.ts
import { load as run } from "./load";
run();
```

### TSX

#### Don't

```tsx
export default function load() {}
```

#### Do

```tsx
export function load() {}
```

#### Don't

```tsx
// src/load.tsx
export default function load() {}
// src/app.tsx
import run from "./load";
run();
```

#### Do

```tsx
// src/load.tsx
export function load() {}
// src/app.tsx
import { load as run } from "./load";
run();
```
