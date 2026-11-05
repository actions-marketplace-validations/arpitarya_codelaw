---
id: js/no-new-array
title: "No new Array(n)"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "a length and a single element give different arrays: both rewrites are offered and the reviewer picks the one that was meant"
stability: preview
ported_from: [eslint-plugin-unicorn/no-new-array]
since: 0.5.0
---

`new Array` is not called with one argument.

**Why:** `new Array(3)` makes three empty slots but `new Array("3")` an array holding "3": one call, two meanings.

```codelaw
match:
  pattern: "new Array($N)"
message: "Use `Array.from({ length })` or an array literal"
fix:
  choices:
    - label: a length
      fix: "Array.from({ length: $N })"
    - label: one element
      fix: "[$N]"
```

## How to fix

1. For a length, write `Array.from({ length: n })`; for one element, `[x]`.

## Examples

### JavaScript

#### Don't

```js
const slots = new Array(size);
```

#### Fixed (a length)

```js
const slots = Array.from({ length: size });
```

#### Fixed (one element)

```js
const slots = [size];
```

#### Do

```js
const slots = Array.from({ length: size });
```

### JSX

#### Don't

```jsx
const slots = new Array(size);
```

#### Fixed (a length)

```jsx
const slots = Array.from({ length: size });
```

#### Fixed (one element)

```jsx
const slots = [size];
```

#### Do

```jsx
const slots = Array.from({ length: size });
```

### TypeScript

#### Don't

```ts
const slots = new Array(size);
```

#### Fixed (a length)

```ts
const slots = Array.from({ length: size });
```

#### Fixed (one element)

```ts
const slots = [size];
```

#### Do

```ts
const slots = Array.from({ length: size });
```

### TSX

#### Don't

```tsx
const slots = new Array(size);
```

#### Fixed (a length)

```tsx
const slots = Array.from({ length: size });
```

#### Fixed (one element)

```tsx
const slots = [size];
```

#### Do

```tsx
const slots = Array.from({ length: size });
```
