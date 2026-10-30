---
id: js/no-useless-rename
title: "No rename to the same name"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint/no-useless-rename]
since: 0.5.0
---

An import, export or destructuring does not rename a name to itself (`{ a as a }`, `{ a: a }`).

**Why:** The rename does nothing.

```codelaw
match:
  any:
  - kind: import_specifier
    all:
    - has: {field: name, pattern: $N}
    - has: {field: alias, pattern: $N}
  - kind: export_specifier
    all:
    - has: {field: name, pattern: $N}
    - has: {field: alias, pattern: $N}
  - kind: pair_pattern
    all:
    - has: {field: key, kind: property_identifier, pattern: $N}
    - has: {field: value, kind: identifier, pattern: $N}
message: '`$N` is renamed to itself'
fix: $N
```

## Examples

### JavaScript

#### Don't

```js
import { readFile as readFile } from 'node:fs';
```

#### Do

```js
import { readFile } from 'node:fs';
```

#### Fixed

```js
import { readFile } from 'node:fs';
```

### JSX

#### Don't

```jsx
import { readFile as readFile } from 'node:fs';
```

#### Do

```jsx
import { readFile } from 'node:fs';
```

#### Fixed

```jsx
import { readFile } from 'node:fs';
```

### TypeScript

#### Don't

```ts
import { readFile as readFile } from 'node:fs';
```

#### Do

```ts
import { readFile } from 'node:fs';
```

#### Fixed

```ts
import { readFile } from 'node:fs';
```

### TSX

#### Don't

```tsx
import { readFile as readFile } from 'node:fs';
```

#### Do

```tsx
import { readFile } from 'node:fs';
```

#### Fixed

```tsx
import { readFile } from 'node:fs';
```
