---
id: js/no-array-splice
title: "toSpliced() rather than splice() for a copy"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "`.toSpliced()` returns the new array rather than the removed items, so the surrounding code must change"
stability: preview
ported_from: [eslint-plugin-unicorn/no-array-splice]
since: 0.5.0
---

`.splice()` is not used; `.toSpliced()` returns a changed copy.

**Why:** `.splice()` both mutates the array and returns the removed items, which is easy to misread.

```codelaw

match:
  pattern: $A.splice($$$X)
message: "`.splice()` mutates `$A`; use `.toSpliced()` for a changed copy, or `.slice()` to read a part"
```

## How to fix

1. If the removed items are what is wanted, use `.slice()`; if the changed array is, assign `.toSpliced(...)` to it.

## Examples

### JavaScript

#### Don't

```js
items.splice(index, 1);
```

#### Do

```js
items = items.toSpliced(index, 1);
```

### JSX

#### Don't

```jsx
items.splice(index, 1);
```

#### Do

```jsx
items = items.toSpliced(index, 1);
```

### TypeScript

#### Don't

```ts
items.splice(index, 1);
```

#### Do

```ts
items = items.toSpliced(index, 1);
```

### TSX

#### Don't

```tsx
items.splice(index, 1);
```

#### Do

```tsx
items = items.toSpliced(index, 1);
```
