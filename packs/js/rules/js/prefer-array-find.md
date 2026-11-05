---
id: js/prefer-array-find
title: "find() rather than filter()[0]"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-array-find]
since: 0.5.0
---

`.filter(f)[0]` is written `.find(f)`.

**Why:** `find` stops at the first match.

```codelaw
match:
  pattern: "$A.filter($F)[0]"
message: "Use `.find()`"
fix: "$A.find($F)"
```

## Examples

### JavaScript

#### Don't

```js
const admin = users.filter((u) => u.admin)[0];
```

#### Do

```js
const admin = users.find((u) => u.admin);
```

### JSX

#### Don't

```jsx
const admin = users.filter((u) => u.admin)[0];
```

#### Do

```jsx
const admin = users.find((u) => u.admin);
```

### TypeScript

#### Don't

```ts
const admin = users.filter((u) => u.admin)[0];
```

#### Do

```ts
const admin = users.find((u) => u.admin);
```

### TSX

#### Don't

```tsx
const admin = users.filter((u) => u.admin)[0];
```

#### Do

```tsx
const admin = users.find((u) => u.admin);
```
