---
id: js/prefer-object-from-entries
title: "Object.fromEntries() rather than reducing pairs"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-object-from-entries]
since: 0.5.0
---

An object built from a list of key-value pairs is built with `Object.fromEntries(pairs)`, not by reducing the pairs into a spread object or with lodash's `_.fromPairs`.

**Why:** `Object.fromEntries` names the operation, and does not copy the object once per pair as the spread form does.

```codelaw
match:
  any:
    - pattern: "$P.reduce(($O, [$K, $V]) => ({ ...$O, [$K]: $V }), {})"
    - pattern: "_.fromPairs($P)"
message: "Use `Object.fromEntries($P)`"
fix: "Object.fromEntries($P)"
```

## Examples

### JavaScript

#### Don't

```js
const byId = pairs.reduce((all, [id, user]) => ({ ...all, [id]: user }), {});
```

#### Do

```js
const byId = Object.fromEntries(pairs);
```


### JSX

#### Don't

```jsx
const byId = pairs.reduce((all, [id, user]) => ({ ...all, [id]: user }), {});
```

#### Do

```jsx
const byId = Object.fromEntries(pairs);
```


### TypeScript

#### Don't

```ts
const byId = pairs.reduce((all, [id, user]) => ({ ...all, [id]: user }), {});
```

#### Do

```ts
const byId = Object.fromEntries(pairs);
```


### TSX

#### Don't

```tsx
const byId = pairs.reduce((all, [id, user]) => ({ ...all, [id]: user }), {});
```

#### Do

```tsx
const byId = Object.fromEntries(pairs);
```
