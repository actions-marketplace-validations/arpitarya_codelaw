---
id: react/jsx-no-undef
title: "Components used in JSX are declared"
languages: [javascript, jsx, tsx]
fix: suggest
fix_reason: "where a project module exports the component, the fix imports it from there (the nearest first; choices when several do); a diff to accept, as the right one may be declared here or named otherwise. A component no project module exports offers no fix (SR-MULTI-FILE-FIXES)"
stability: preview
ported_from: [eslint-plugin-react/jsx-no-undef]
since: 0.1.0
---

Every component used as a JSX tag is imported or declared.

**Why:** An undeclared component throws a ReferenceError when it renders.

```codelaw
native: react/jsx-no-undef
```

## How to fix

1. Where an import is offered, check it and accept it: `codelaw fix --unsafe` (with choices, pick the module).
2. Otherwise import the component, or declare it, or correct the name.

## Examples

### JavaScript

#### Don't

```js
// src/header.jsx
export const Header = () => <h1 />;
// src/app.jsx
export const App = () => <Header />;
```

#### Do

```js
// src/header.jsx
export const Header = () => <h1 />;
// src/app.jsx
import { Header } from "./header";
export const App = () => <Header />;
```

### JSX

#### Don't

```jsx
// src/header.jsx
export const Header = () => <h1 />;
// src/app.jsx
export const App = () => <Header />;
```

#### Do

```jsx
// src/header.jsx
export const Header = () => <h1 />;
// src/app.jsx
import { Header } from "./header";
export const App = () => <Header />;
```

### TSX

#### Don't

```tsx
// src/header.tsx
export const Header = () => <h1 />;
// src/app.tsx
export const App = () => <Header />;
```

#### Do

```tsx
// src/header.tsx
export const Header = () => <h1 />;
// src/app.tsx
import { Header } from "./header";
export const App = () => <Header />;
```
