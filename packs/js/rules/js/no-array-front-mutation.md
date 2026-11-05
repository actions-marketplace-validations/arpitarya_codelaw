---
id: js/no-array-front-mutation
title: "No unshift() or shift()"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "switching ends changes the order the rest of the code reads"
stability: preview
ported_from: [eslint-plugin-unicorn/no-array-front-mutation]
since: 0.5.0
---

Elements are not added to the front of an array with `unshift`.

**Why:** Adding to the front shifts every element, so it is O(n), and the order logic is easy to get backwards.

```codelaw

match:
  pattern: $A.unshift($$$X)
message: "`unshift` moves every element; push to the end, or build a new array"
```

## How to fix

1. Push at the end and read from the end, or build a new array `[item, ...queue]` where a copy is wanted.

## Examples

### JavaScript

#### Don't

```js
queue.unshift(job);
```

#### Do

```js
queue.push(job);
```

### JSX

#### Don't

```jsx
queue.unshift(job);
```

#### Do

```jsx
queue.push(job);
```

### TypeScript

#### Don't

```ts
queue.unshift(job);
```

#### Do

```ts
queue.push(job);
```

### TSX

#### Don't

```tsx
queue.unshift(job);
```

#### Do

```tsx
queue.push(job);
```
