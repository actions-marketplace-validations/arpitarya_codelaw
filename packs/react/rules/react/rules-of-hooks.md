---
id: react/rules-of-hooks
title: "Hooks are called at the top level of components and hooks"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "a hook called after an early return moves above the statement holding it, when nothing it reads or declares sits in between; it then runs on renders that returned early, which is the fix but is a change. Every other misplaced hook is manual"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-react-hooks/rules-of-hooks, eslint-plugin-react-hooks/hooks]
since: 0.1.0
---

Hooks (`useState`, `useEffect`, any `use…`) are called only at the top level of a function component
or a custom hook: not in conditions, loops, nested functions, after an early return, in classes, or
in plain functions.

**Why:** React matches hook calls to state by their order, so a hook that runs on some renders and
not others mixes up every hook after it.

```codelaw
native: react/rules-of-hooks
```

## How to fix

1. Move the hook call to the top level of the component or hook, before any early return.
2. Put the condition inside the hook (e.g. inside the effect) instead of around it.

## Examples

### JavaScript

#### Don't

```js
function Profile({ id }) {
  if (!id) {
    return null;
  }
  const [user] = useState(null);
  return user;
}
```

#### Do

```js
function Profile({ id }) {
  const [user] = useState(null);
  if (!id) {
    return null;
  }
  return user;
}
```

### JSX

#### Don't

```jsx
function Profile({ id }) {
  if (!id) {
    return null;
  }
  const [user] = useState(null);
  return user;
}
```

#### Do

```jsx
function Profile({ id }) {
  const [user] = useState(null);
  if (!id) {
    return null;
  }
  return user;
}
```

### TypeScript

#### Don't

```ts
function Profile({ id }) {
  if (!id) {
    return null;
  }
  const [user] = useState(null);
  return user;
}
```

#### Do

```ts
function Profile({ id }) {
  const [user] = useState(null);
  if (!id) {
    return null;
  }
  return user;
}
```

### TSX

#### Don't

```tsx
function Profile({ id }) {
  if (!id) {
    return null;
  }
  const [user] = useState(null);
  return user;
}
```

#### Do

```tsx
function Profile({ id }) {
  const [user] = useState(null);
  if (!id) {
    return null;
  }
  return user;
}
```
