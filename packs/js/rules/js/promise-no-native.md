---
id: js/promise-no-native
title: "Promise is imported, not the global"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "which promise library to import is the project's choice"
fix_exception: value
stability: preview
ported_from: [eslint-plugin-promise/no-native]
since: 0.5.0
---

`Promise` is used only where it is declared or imported in the file, for code that must run where there is no native `Promise`.

**Why:** In an environment without a native `Promise`, the global one is `undefined` and the code fails at the first use.

```codelaw
match:
  kind: identifier
  regex: '^Promise$'
where:
  $$MATCH: builtin
message: "`Promise` is the global; import a promise library for environments without it"
```

## How to fix

1. Import the promise implementation the project uses (`import Promise from "bluebird"`) at the top of the file.

## Examples

### JavaScript

#### Don't

```js
const p = Promise.resolve(1);
```

#### Do

```js
import Promise from "bluebird";
const p = Promise.resolve(1);
```

### JSX

#### Don't

```jsx
const p = Promise.resolve(1);
```

#### Do

```jsx
import Promise from "bluebird";
const p = Promise.resolve(1);
```

### TypeScript

#### Don't

```ts
const p = Promise.resolve(1);
```

#### Do

```ts
import Promise from "bluebird";
const p = Promise.resolve(1);
```

### TSX

#### Don't

```tsx
const p = Promise.resolve(1);
```

#### Do

```tsx
import Promise from "bluebird";
const p = Promise.resolve(1);
```
