---
id: js/no-useless-fallback-in-spread
title: "No || {} in an object spread"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/no-useless-fallback-in-spread]
since: 0.5.0
---

An object spread does not fall back to `{}` with `||` (`...(x || {})`).

**Why:** Spreading `undefined`, `null` or another falsy primitive into an object adds nothing already.

```codelaw

match:
  kind: spread_element
  inside: { kind: object }
  has: { kind: parenthesized_expression, has: { pattern: "$X || {}" } }
message: "Spreading `undefined` adds nothing; drop the fallback"
fix: "...$X"
```

## Examples

### JavaScript

#### Don't

```js
const opts = { ...(input || {}) };
```

#### Do

```js
const opts = { ...input };
```

### JSX

#### Don't

```jsx
const opts = { ...(input || {}) };
```

#### Do

```jsx
const opts = { ...input };
```

### TypeScript

#### Don't

```ts
const opts = { ...(input || {}) };
```

#### Do

```ts
const opts = { ...input };
```

### TSX

#### Don't

```tsx
const opts = { ...(input || {}) };
```

#### Do

```tsx
const opts = { ...input };
```
