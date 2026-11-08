---
id: js/new-cap
title: "Constructors are capitalised"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "the constructor is renamed with a capital where it is declared and wherever it is used, its importers included, as one fix; a diff to accept, as code outside the project may import it too. A global, a member (`new a.b()`) or a name reached by `require` or a namespace offers no rename (SR-MULTI-FILE-FIXES)"
stability: preview
ported_from: [eslint/new-cap]
since: 0.5.0
---

A function called with `new` has a name starting with a capital letter. Upstream also reports capitalised functions called without `new`; this port checks `new`.

**Why:** The capital is the only sign at the call site that `new` is required.

```codelaw
native: js/new-cap
```

## How to fix

1. Accept the rename: `codelaw fix --unsafe` renames the constructor where it is declared and wherever it is used (the diff names each file).
2. A constructor this project doesn't declare (a global, a member, a package's) offers no rename: rename it by hand, or keep it and suppress the line with a reason.

## Examples

### JavaScript

#### Don't

```js
class user {}
const u = new user();
```

#### Do

```js
class User {}
const u = new User();
```

#### Don't

```js
// src/user.js
export class user {}
// src/app.js
import { user } from "./user";
export const u = new user();
```

#### Do

```js
// src/user.js
export class User {}
// src/app.js
import { User } from "./user";
export const u = new User();
```

### JSX

#### Don't

```jsx
class user {}
const u = new user();
```

#### Do

```jsx
class User {}
const u = new User();
```

#### Don't

```jsx
// src/user.jsx
export class user {}
// src/app.jsx
import { user } from "./user";
export const u = new user();
```

#### Do

```jsx
// src/user.jsx
export class User {}
// src/app.jsx
import { User } from "./user";
export const u = new User();
```

### TypeScript

#### Don't

```ts
class user {}
const u = new user();
```

#### Do

```ts
class User {}
const u = new User();
```

#### Don't

```ts
// src/user.ts
export class user {}
// src/app.ts
import { user } from "./user";
export const u = new user();
```

#### Do

```ts
// src/user.ts
export class User {}
// src/app.ts
import { User } from "./user";
export const u = new User();
```

### TSX

#### Don't

```tsx
class user {}
const u = new user();
```

#### Do

```tsx
class User {}
const u = new User();
```

#### Don't

```tsx
// src/user.tsx
export class user {}
// src/app.tsx
import { user } from "./user";
export const u = new user();
```

#### Do

```tsx
// src/user.tsx
export class User {}
// src/app.tsx
import { User } from "./user";
export const u = new User();
```
