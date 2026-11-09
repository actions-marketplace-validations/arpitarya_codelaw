---
id: js/no-undef
title: "No references to undeclared names"
languages: [javascript, jsx]
fix: suggest
fix_reason: "where a project module exports the name, the fix imports it from there (the nearest first; choices when several do); a diff to accept, as the right declaration may be a local or an environment instead. A name no project module exports offers no fix (SR-MULTI-FILE-FIXES)"
stability: preview
options:
  env: { type: "string[]", default: ["shared-node-browser"], description: "environments whose globals are declared: browser, node, commonjs, worker, serviceworker, shared-node-browser, jest, mocha, vitest" }
  globals: { type: "string[]", default: [], description: "more names to treat as declared" }
  typeof: { type: "boolean", default: false, description: "also report names used only as `typeof x`" }
ported_from: [eslint/no-undef, eslint-plugin-sonarjs/no-implicit-global, eslint-plugin-sonarjs/no-reference-error]
since: 0.1.0
---

Every name the code reads or writes is declared in the file, built into JavaScript, or a global of
an environment the rule's `env` option names. As in ESLint, a `/* global a, b:writable */` comment
declares names for that file (`name:off` declares nothing), and a `/* eslint-env node */` comment
adds environments.

**Why:** An undeclared name is usually a typo or a missing import, and throws a ReferenceError when
that line runs.

```codelaw
native: js/no-undef
```

## How to fix

1. Where an import is offered, check it and accept it: `codelaw fix --unsafe` (with choices, pick the module).
2. If it's a typo, correct the name.
3. If it's a global of the runtime (browser, Node), add that environment to the rule's `env` option.

## Examples

### JavaScript

#### Don't

```js
// src/math.js
export function add(a, b) {
  return a + b;
}
// src/app.js
export const total = [1, 2].reduce(add, 0);
```

#### Do

```js
// src/math.js
export function add(a, b) {
  return a + b;
}
// src/app.js
import { add } from "./math";
export const total = [1, 2].reduce(add, 0);
```

### JSX

#### Don't

```jsx
// src/math.jsx
export function add(a, b) {
  return a + b;
}
// src/app.jsx
export const total = [1, 2].reduce(add, 0);
```

#### Do

```jsx
// src/math.jsx
export function add(a, b) {
  return a + b;
}
// src/app.jsx
import { add } from "./math";
export const total = [1, 2].reduce(add, 0);
```
