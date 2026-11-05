---
id: js/prefer-array-flat
title: "flat() rather than concat spreading"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-array-flat]
since: 0.5.0
---

`[].concat(...a)` is written `a.flat()`. Upstream also covers `reduce` and `_.flatten`; this port covers the concat form.

**Why:** `flat()` names the operation.

```codelaw
match:
  pattern: "[].concat(...$A)"
message: "Use `.flat()`"
fix: "$A.flat()"
```

## Examples

### JavaScript

#### Don't

```js
const flat = [].concat(...rows);
```

#### Do

```js
const flat = rows.flat();
```

### JSX

#### Don't

```jsx
const flat = [].concat(...rows);
```

#### Do

```jsx
const flat = rows.flat();
```

### TypeScript

#### Don't

```ts
const flat = [].concat(...rows);
```

#### Do

```ts
const flat = rows.flat();
```

### TSX

#### Don't

```tsx
const flat = [].concat(...rows);
```

#### Do

```tsx
const flat = rows.flat();
```
