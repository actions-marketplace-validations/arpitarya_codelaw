---
id: react/jsx-pascal-case
title: "Component names are PascalCase"
languages: [javascript, jsx, tsx]
fix: suggest
fix_reason: "the component is renamed in PascalCase where it is declared and wherever it is used, its importers and closing tags included, as one fix; a diff to accept, as code outside the project may import it. A component this project doesn't declare offers no rename (SR-MULTI-FILE-FIXES)"
stability: preview
ported_from: [eslint-plugin-react/jsx-pascal-case]
since: 0.5.0
---

A component named in JSX is PascalCase: no underscores (`<User_Card>`) and not all capitals (`<USERCARD>`).

**Why:** JSX reads a capitalised name as a component; one casing for all of them keeps imports and file names predictable.

```codelaw
native: react/jsx-pascal-case
```

## How to fix

1. Accept the rename: `codelaw fix --unsafe` renames the component where it is declared and wherever it is used (the diff names each file).
2. A component this project doesn't declare offers no rename: rename it by hand.

## Examples

### JavaScript

#### Don't

```js
function User_Card() {
  return <p />;
}
const c = <User_Card />;
```

#### Do

```js
function UserCard() {
  return <p />;
}
const c = <UserCard />;
```

#### Don't

```js
// src/card.jsx
export function User_Card() {
  return <p />;
}
// src/app.jsx
import { User_Card } from "./card";
export const App = () => <User_Card></User_Card>;
```

#### Do

```js
// src/card.jsx
export function UserCard() {
  return <p />;
}
// src/app.jsx
import { UserCard } from "./card";
export const App = () => <UserCard></UserCard>;
```

### JSX

#### Don't

```jsx
function User_Card() {
  return <p />;
}
const c = <User_Card />;
```

#### Do

```jsx
function UserCard() {
  return <p />;
}
const c = <UserCard />;
```

#### Don't

```jsx
// src/card.jsx
export function User_Card() {
  return <p />;
}
// src/app.jsx
import { User_Card } from "./card";
export const App = () => <User_Card></User_Card>;
```

#### Do

```jsx
// src/card.jsx
export function UserCard() {
  return <p />;
}
// src/app.jsx
import { UserCard } from "./card";
export const App = () => <UserCard></UserCard>;
```

### TSX

#### Don't

```tsx
function User_Card() {
  return <p />;
}
const c = <User_Card />;
```

#### Do

```tsx
function UserCard() {
  return <p />;
}
const c = <UserCard />;
```

#### Don't

```tsx
// src/card.tsx
export function User_Card() {
  return <p />;
}
// src/app.tsx
import { User_Card } from "./card";
export const App = () => <User_Card></User_Card>;
```

#### Do

```tsx
// src/card.tsx
export function UserCard() {
  return <p />;
}
// src/app.tsx
import { UserCard } from "./card";
export const App = () => <UserCard></UserCard>;
```
