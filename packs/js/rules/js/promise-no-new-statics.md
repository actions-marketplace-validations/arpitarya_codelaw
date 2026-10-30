---
id: js/promise-no-new-statics
title: "Promise's static methods are called without new"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-promise/no-new-statics]
since: 0.5.0
---

`Promise.resolve`, `Promise.reject`, `Promise.all` and the other static methods are called as functions, never with `new`.

**Why:** `new Promise.resolve(x)` throws a `TypeError` at run time: the statics are not constructors.

```codelaw
match:
  pattern: new Promise.$M($$$ARGS)
constraints:
  M: { regex: '^(resolve|reject|all|allSettled|race|any|withResolvers|try)$' }
message: "`Promise.$M` is not a constructor; call it without `new`"
fix: "Promise.$M($$$ARGS)"
```

## Examples

### JavaScript

#### Don't

```js
const p = new Promise.resolve(1);
```

#### Do

```js
const p = Promise.resolve(1);
```

### JSX

#### Don't

```jsx
const p = new Promise.resolve(1);
```

#### Do

```jsx
const p = Promise.resolve(1);
```

### TypeScript

#### Don't

```ts
const p = new Promise.resolve(1);
```

#### Do

```ts
const p = Promise.resolve(1);
```

### TSX

#### Don't

```tsx
const p = new Promise.resolve(1);
```

#### Do

```tsx
const p = Promise.resolve(1);
```
