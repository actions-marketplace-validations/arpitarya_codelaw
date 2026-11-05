---
id: js/require-number-to-fixed-digits-argument
title: "toFixed() names its digits"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/require-number-to-fixed-digits-argument]
since: 0.5.0
---

`.toFixed()` is called with its digits argument.

**Why:** The default 0 reads as a forgotten argument.

```codelaw
match:
  pattern: "$N.toFixed()"
message: "Pass the number of digits to `.toFixed()`"
fix: "$N.toFixed(0)"
```

## Examples

### JavaScript

#### Don't

```js
const s = price.toFixed();
```

#### Do

```js
const s = price.toFixed(0);
```

### JSX

#### Don't

```jsx
const s = price.toFixed();
```

#### Do

```jsx
const s = price.toFixed(0);
```

### TypeScript

#### Don't

```ts
const s = price.toFixed();
```

#### Do

```ts
const s = price.toFixed(0);
```

### TSX

#### Don't

```tsx
const s = price.toFixed();
```

#### Do

```tsx
const s = price.toFixed(0);
```
