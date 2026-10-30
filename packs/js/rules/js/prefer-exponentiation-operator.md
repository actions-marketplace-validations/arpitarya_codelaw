---
id: js/prefer-exponentiation-operator
title: "** rather than Math.pow"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint/prefer-exponentiation-operator]
since: 0.5.0
---

`a ** b` is written instead of `Math.pow(a, b)`.

**Why:** The operator is shorter and reads like the arithmetic it is.

```codelaw
match:
  pattern: Math.pow($A, $B)
message: "Use the `**` operator instead of `Math.pow`"
fix: "($A) ** ($B)"
```

## Examples

### JavaScript

#### Don't

```js
const area = Math.pow(r, 2);
```

#### Do

```js
const area = r ** 2;
```

#### Fixed

```js
const area = (r) ** (2);
```

### JSX

#### Don't

```jsx
const area = Math.pow(r, 2);
```

#### Do

```jsx
const area = r ** 2;
```

#### Fixed

```jsx
const area = (r) ** (2);
```

### TypeScript

#### Don't

```ts
const area = Math.pow(r, 2);
```

#### Do

```ts
const area = r ** 2;
```

#### Fixed

```ts
const area = (r) ** (2);
```

### TSX

#### Don't

```tsx
const area = Math.pow(r, 2);
```

#### Do

```tsx
const area = r ** 2;
```

#### Fixed

```tsx
const area = (r) ** (2);
```
