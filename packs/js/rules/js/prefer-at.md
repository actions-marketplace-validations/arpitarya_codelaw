---
id: js/prefer-at
title: "at(-1) rather than [a.length - 1]"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-at]
since: 0.5.0
---

`a[a.length - 1]` is written `a.at(-1)`.

**Why:** `.at` names the array once and reads as "the last".

```codelaw
match:
  pattern: "$A[$A.length - 1]"
  not: { inside: { kind: assignment_expression, field: left } }
message: "Use `.at(-1)`"
fix: "$A.at(-1)"
```

## Examples

### JavaScript

#### Don't

```js
const last = items[items.length - 1];
```

#### Do

```js
const last = items.at(-1);
```

### JSX

#### Don't

```jsx
const last = items[items.length - 1];
```

#### Do

```jsx
const last = items.at(-1);
```

### TypeScript

#### Don't

```ts
const last = items[items.length - 1];
```

#### Do

```ts
const last = items.at(-1);
```

### TSX

#### Don't

```tsx
const last = items[items.length - 1];
```

#### Do

```tsx
const last = items.at(-1);
```
