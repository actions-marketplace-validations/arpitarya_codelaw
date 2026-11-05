---
id: js/no-unnecessary-slice-end
title: "No slice(a, x.length)"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/no-unnecessary-slice-end]
since: 0.5.0
---

`.slice(start, a.length)` is written `.slice(start)`.

**Why:** The length is the default end.

```codelaw
match:
  pattern: "$A.slice($S, $A.length)"
message: "The end is the default; drop it"
fix: "$A.slice($S)"
```

## Examples

### JavaScript

#### Don't

```js
const rest = list.slice(1, list.length);
```

#### Do

```js
const rest = list.slice(1);
```

### JSX

#### Don't

```jsx
const rest = list.slice(1, list.length);
```

#### Do

```jsx
const rest = list.slice(1);
```

### TypeScript

#### Don't

```ts
const rest = list.slice(1, list.length);
```

#### Do

```ts
const rest = list.slice(1);
```

### TSX

#### Don't

```tsx
const rest = list.slice(1, list.length);
```

#### Do

```tsx
const rest = list.slice(1);
```
