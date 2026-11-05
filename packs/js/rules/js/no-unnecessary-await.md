---
id: js/no-unnecessary-await
title: "No await on a value that is not a promise"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/no-unnecessary-await]
since: 0.5.0
---

`await` is not applied to a literal (a number, string, array, object or function).

**Why:** Awaiting a non-promise only delays the code by a tick.

```codelaw
match:
  pattern: "await $X"
constraints:
  X: { any: [{kind: number}, {kind: string}, {kind: array}, {kind: object}, {kind: arrow_function}, {kind: template_string}] }
message: "This value is not a promise; drop the `await`"
fix: "$X"
```

## Examples

### JavaScript

#### Don't

```js
const n = await 3;
```

#### Do

```js
const n = 3;
```

### JSX

#### Don't

```jsx
const n = await 3;
```

#### Do

```jsx
const n = 3;
```

### TypeScript

#### Don't

```ts
const n = await 3;
```

#### Do

```ts
const n = 3;
```

### TSX

#### Don't

```tsx
const n = await 3;
```

#### Do

```tsx
const n = 3;
```
