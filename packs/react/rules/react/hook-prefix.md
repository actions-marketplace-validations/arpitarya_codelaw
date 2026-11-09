---
id: react/hook-prefix
title: "Functions that call hooks are named use…"
languages: [javascript, jsx, typescript, tsx]
fix: auto
fix_reason: "auto when nothing imports the function: it is renamed `use…` with every call in its module; an exported one is renamed in every project module that imports it too, as one fix, a diff to accept, as code outside the project may import it (SR-MULTI-FILE-FIXES)"
stability: preview
since: 0.1.0
---

A function that isn't a component (PascalCase) and calls a hook is a custom hook, and is named
`use…`.

**Why:** React and its tools recognise hooks by the `use` prefix; without it the Rules of Hooks
can't be checked for the function's callers.

```codelaw
native: react/hook-prefix
```

## How to fix

1. `codelaw fix` renames the function `use…` with every call to it; for an exported one, check the diff and accept it with `codelaw fix --unsafe`.
2. A name taken already, or an importer that reaches it through a namespace, `require` or a re-export, offers no rename: rename it by hand.

## Examples

### JavaScript

#### Don't

```js
function currentUser() {
  return useContext(UserContext);
}
```

#### Do

```js
function useCurrentUser() {
  return useContext(UserContext);
}
```

#### Don't

```js
// src/user.js
export function currentUser() {
  return useContext(UserContext);
}
// src/app.js
import { currentUser } from "./user";
export const u = () => currentUser();
```

#### Do

```js
// src/user.js
export function useCurrentUser() {
  return useContext(UserContext);
}
// src/app.js
import { useCurrentUser } from "./user";
export const u = () => useCurrentUser();
```

### JSX

#### Don't

```jsx
function currentUser() {
  return useContext(UserContext);
}
```

#### Do

```jsx
function useCurrentUser() {
  return useContext(UserContext);
}
```

#### Don't

```jsx
// src/user.jsx
export function currentUser() {
  return useContext(UserContext);
}
// src/app.jsx
import { currentUser } from "./user";
export const u = () => currentUser();
```

#### Do

```jsx
// src/user.jsx
export function useCurrentUser() {
  return useContext(UserContext);
}
// src/app.jsx
import { useCurrentUser } from "./user";
export const u = () => useCurrentUser();
```

### TypeScript

#### Don't

```ts
function currentUser() {
  return useContext(UserContext);
}
```

#### Do

```ts
function useCurrentUser() {
  return useContext(UserContext);
}
```

#### Don't

```ts
// src/user.ts
export function currentUser() {
  return useContext(UserContext);
}
// src/app.ts
import { currentUser } from "./user";
export const u = () => currentUser();
```

#### Do

```ts
// src/user.ts
export function useCurrentUser() {
  return useContext(UserContext);
}
// src/app.ts
import { useCurrentUser } from "./user";
export const u = () => useCurrentUser();
```

### TSX

#### Don't

```tsx
function currentUser() {
  return useContext(UserContext);
}
```

#### Do

```tsx
function useCurrentUser() {
  return useContext(UserContext);
}
```

#### Don't

```tsx
// src/user.tsx
export function currentUser() {
  return useContext(UserContext);
}
// src/app.tsx
import { currentUser } from "./user";
export const u = () => currentUser();
```

#### Do

```tsx
// src/user.tsx
export function useCurrentUser() {
  return useContext(UserContext);
}
// src/app.tsx
import { useCurrentUser } from "./user";
export const u = () => useCurrentUser();
```
