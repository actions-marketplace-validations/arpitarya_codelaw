---
id: js/prefer-string-repeat
title: "s.repeat(n) rather than Array(n + 1).join(s)"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-string-repeat]
since: 0.5.0
---

`new Array(n + 1).join(s)` is written `s.repeat(n)`.

**Why:** `repeat` names the operation.

```codelaw
match:
  pattern: "new Array($N + 1).join($S)"
message: "Use `.repeat()`"
fix: "$S.repeat($N)"
```

## Examples

### JavaScript

#### Don't

```js
const bar = new Array(n + 1).join("-");
```

#### Do

```js
const bar = "-".repeat(n);
```

### JSX

#### Don't

```jsx
const bar = new Array(n + 1).join("-");
```

#### Do

```jsx
const bar = "-".repeat(n);
```

### TypeScript

#### Don't

```ts
const bar = new Array(n + 1).join("-");
```

#### Do

```ts
const bar = "-".repeat(n);
```

### TSX

#### Don't

```tsx
const bar = new Array(n + 1).join("-");
```

#### Do

```tsx
const bar = "-".repeat(n);
```
