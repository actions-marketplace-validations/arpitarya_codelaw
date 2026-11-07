---
id: js/prefer-promise-static-methods
title: "Promise.resolve() or Promise.reject() rather than new Promise"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-promise-static-methods]
since: 0.5.0
---

A promise that only settles with a value already at hand is `Promise.resolve(value)` or `Promise.reject(reason)`, not `new Promise((resolve) => resolve(value))`.

**Why:** The static method says what the promise is without an executor to read.

```codelaw
utils:
  plain:
    any:
      - kind: identifier
      - kind: string
      - kind: number
      - kind: "null"
      - kind: "true"
      - kind: "false"
match:
  any:
    - pattern: "new Promise(($R) => $R($V))"
      all: [{ pattern: $RES }]
    - pattern: "new Promise(($R, $J) => $R($V))"
      all: [{ pattern: $RES }]
    - pattern: "new Promise(($R, $J) => $J($W))"
      all: [{ pattern: $REJ }]
constraints:
  V: { matches: plain }
  W: { matches: plain }
message: "Use the static method"
fix:
  - replace: { target: $RES, with: "Promise.resolve($V)" }
  - replace: { target: $REJ, with: "Promise.reject($W)" }
```

## Examples

### JavaScript

#### Don't

```js
const ready = new Promise((resolve) => resolve(cached));
const failed = new Promise((_, reject) => reject(error));
```

#### Do

```js
const ready = Promise.resolve(cached);
const failed = Promise.reject(error);
```


### JSX

#### Don't

```jsx
const ready = new Promise((resolve) => resolve(cached));
const failed = new Promise((_, reject) => reject(error));
```

#### Do

```jsx
const ready = Promise.resolve(cached);
const failed = Promise.reject(error);
```


### TypeScript

#### Don't

```ts
const ready = new Promise((resolve) => resolve(cached));
const failed = new Promise((_, reject) => reject(error));
```

#### Do

```ts
const ready = Promise.resolve(cached);
const failed = Promise.reject(error);
```


### TSX

#### Don't

```tsx
const ready = new Promise((resolve) => resolve(cached));
const failed = new Promise((_, reject) => reject(error));
```

#### Do

```tsx
const ready = Promise.resolve(cached);
const failed = Promise.reject(error);
```
