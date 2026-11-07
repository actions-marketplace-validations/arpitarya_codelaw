---
id: js/prefer-array-last-methods
title: "findLast() rather than reversing a copy to find"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "the callback's index argument counts from the other end after the rewrite; offered for review"
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-array-last-methods]
since: 0.5.0
---

The last element that matches is found with `arr.findLast(fn)`, not by reversing a copy and calling `find`.

**Why:** `findLast` walks from the end without copying the array.

```codelaw
match:
  any:
    - pattern: "[...$A].reverse().find($F)"
    - pattern: "$A.slice().reverse().find($F)"
    - pattern: "$A.toReversed().find($F)"
message: "Use `$A.findLast(...)`"
fix: "$A.findLast($F)"
```

## Examples

### JavaScript

#### Don't

```js
const last = [...events].reverse().find((e) => e.ok);
```

#### Do

```js
const last = events.findLast((e) => e.ok);
```


### JSX

#### Don't

```jsx
const last = [...events].reverse().find((e) => e.ok);
```

#### Do

```jsx
const last = events.findLast((e) => e.ok);
```


### TypeScript

#### Don't

```ts
const last = [...events].reverse().find((e) => e.ok);
```

#### Do

```ts
const last = events.findLast((e) => e.ok);
```


### TSX

#### Don't

```tsx
const last = [...events].reverse().find((e) => e.ok);
```

#### Do

```tsx
const last = events.findLast((e) => e.ok);
```
