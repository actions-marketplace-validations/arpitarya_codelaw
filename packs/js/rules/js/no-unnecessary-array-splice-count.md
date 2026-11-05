---
id: js/no-unnecessary-array-splice-count
title: "No length as splice count"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/no-unnecessary-array-splice-count]
since: 0.5.0
---

`.splice(i, a.length)` is written `.splice(i)`.

**Why:** Without a count, `splice` removes to the end already.

```codelaw
match:
  pattern: "$A.splice($I, $A.length)"
message: "The count is the default; drop it"
fix: "$A.splice($I)"
```

## Examples

### JavaScript

#### Don't

```js
list.splice(2, list.length);
```

#### Do

```js
list.splice(2);
```

### JSX

#### Don't

```jsx
list.splice(2, list.length);
```

#### Do

```jsx
list.splice(2);
```

### TypeScript

#### Don't

```ts
list.splice(2, list.length);
```

#### Do

```ts
list.splice(2);
```

### TSX

#### Don't

```tsx
list.splice(2, list.length);
```

#### Do

```tsx
list.splice(2);
```
