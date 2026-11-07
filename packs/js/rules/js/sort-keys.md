---
id: js/sort-keys
title: "Object keys in ascending order"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "the object's keys are enumerated in a new order, which code that iterates it may notice"
stability: preview
ported_from: [eslint/sort-keys]
since: 0.5.0
---

The static keys of an object literal are in ascending (code-unit) order, ESLint's default; a spread starts a new run. The first key out of order is reported. The fix sorts the whole object when every key is static, nothing is spread, no comment is inside and each value is a literal, a name or a function; otherwise it is reported without one.

**Why:** Sorted keys are easy to scan and merge without conflicts.

```codelaw
native: js/sort-keys
```

## Examples

### JavaScript

#### Don't

```js
const point = { y: 2, x: 1 };
```

#### Do

```js
const point = { x: 1, y: 2 };
```


### JSX

#### Don't

```jsx
const point = { y: 2, x: 1 };
```

#### Do

```jsx
const point = { x: 1, y: 2 };
```


### TypeScript

#### Don't

```ts
const point = { y: 2, x: 1 };
```

#### Do

```ts
const point = { x: 1, y: 2 };
```


### TSX

#### Don't

```tsx
const point = { y: 2, x: 1 };
```

#### Do

```tsx
const point = { x: 1, y: 2 };
```
