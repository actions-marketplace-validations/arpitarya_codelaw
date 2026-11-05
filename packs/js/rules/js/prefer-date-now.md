---
id: js/prefer-date-now
title: "Date.now() rather than new Date().getTime()"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-date-now]
since: 0.5.0
---

`new Date().getTime()` is written `Date.now()`.

**Why:** `Date.now()` reads the time without allocating a `Date`.

```codelaw
match:
  pattern: "new Date().getTime()"
message: "Use `Date.now()`"
fix: "Date.now()"
```

## Examples

### JavaScript

#### Don't

```js
const t = new Date().getTime();
```

#### Do

```js
const t = Date.now();
```

### JSX

#### Don't

```jsx
const t = new Date().getTime();
```

#### Do

```jsx
const t = Date.now();
```

### TypeScript

#### Don't

```ts
const t = new Date().getTime();
```

#### Do

```ts
const t = Date.now();
```

### TSX

#### Don't

```tsx
const t = new Date().getTime();
```

#### Do

```tsx
const t = Date.now();
```
