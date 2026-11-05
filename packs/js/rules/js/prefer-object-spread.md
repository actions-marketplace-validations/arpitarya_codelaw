---
id: js/prefer-object-spread
title: "{ ...a } rather than Object.assign({}, a)"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "`Object.assign` runs setters on the target and a spread defines properties, which differs for a getter-only source"
stability: preview
ported_from: [eslint/prefer-object-spread]
since: 0.5.0
---

`Object.assign` with an empty object literal as its first argument is written as an object spread.

**Why:** The spread says "a new object made of these" directly and is easier to type-check.

```codelaw

match:
  pattern: Object.assign({}, $$$A)
  not: { has: { field: arguments, has: { kind: spread_element } } }
message: "Use an object spread: `{ ...$$$A }`"
fix: "{ ...$$$A }"
```

## Examples

### JavaScript

#### Don't

```js
const merged = Object.assign({}, defaults);
```

#### Do

```js
const merged = { ...defaults };
```

### JSX

#### Don't

```jsx
const merged = Object.assign({}, defaults);
```

#### Do

```jsx
const merged = { ...defaults };
```

### TypeScript

#### Don't

```ts
const merged = Object.assign({}, defaults);
```

#### Do

```ts
const merged = { ...defaults };
```

### TSX

#### Don't

```tsx
const merged = Object.assign({}, defaults);
```

#### Do

```tsx
const merged = { ...defaults };
```
