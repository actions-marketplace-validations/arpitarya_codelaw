---
id: js/prefer-group-by
title: "Object.groupBy() rather than reducing into arrays"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "`Object.groupBy` returns an object with no prototype; code that calls `hasOwnProperty` on the result needs review"
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-group-by]
since: 0.5.0
---

Grouping elements by a key is `Object.groupBy(items, (item) => key)`, not a `reduce` that pushes each element into `acc[key] ??= []` (or `||=`).

**Why:** The built-in names the operation; the reduce makes the reader work it out.

```codelaw
match:
  any:
    - pattern: "$A.reduce(($ACC, $X) => { ($ACC[$K] ??= []).push($X); return $ACC; }, {})"
    - pattern: "$A.reduce(($ACC, $X) => { ($ACC[$K] ||= []).push($X); return $ACC; }, {})"
message: "Use `Object.groupBy`"
fix: "Object.groupBy($A, ($X) => $K)"
```

## Examples

### JavaScript

#### Don't

```js
const byRole = users.reduce((acc, user) => { (acc[user.role] ??= []).push(user); return acc; }, {});
```

#### Do

```js
const byRole = Object.groupBy(users, (user) => user.role);
```


### JSX

#### Don't

```jsx
const byRole = users.reduce((acc, user) => { (acc[user.role] ??= []).push(user); return acc; }, {});
```

#### Do

```jsx
const byRole = Object.groupBy(users, (user) => user.role);
```


### TypeScript

#### Don't

```ts
const byRole = users.reduce((acc, user) => { (acc[user.role] ??= []).push(user); return acc; }, {});
```

#### Do

```ts
const byRole = Object.groupBy(users, (user) => user.role);
```


### TSX

#### Don't

```tsx
const byRole = users.reduce((acc, user) => { (acc[user.role] ??= []).push(user); return acc; }, {});
```

#### Do

```tsx
const byRole = Object.groupBy(users, (user) => user.role);
```
