---
id: js/no-duplicate-imports
title: "One import per module"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "merging the specifier lists is a computed edit"
fix_pending: "auto; needs computed-edit (W-162)"
stability: preview
ported_from: [eslint/no-duplicate-imports]
since: 0.5.0
---

A module is imported by one `import` statement; type-only imports are counted apart.

**Why:** Two imports of the same module are noise and make it easy to import a name twice.

```codelaw

match:
  kind: import_statement
  has: { field: source, pattern: $S }
  not: { regex: '^import\s+type\b' }
  follows: { stopBy: end, kind: import_statement, has: { field: source, pattern: $S }, not: { regex: '^import\s+type\b' } }
message: "`$S` is imported again; merge the two imports"
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JavaScript

#### Don't

```js
import { a } from "./util";
import { b } from "./util";
```

#### Do

```js
import { a, b } from "./util";
```

### JSX

#### Don't

```jsx
import { a } from "./util";
import { b } from "./util";
```

#### Do

```jsx
import { a, b } from "./util";
```

### TypeScript

#### Don't

```ts
import { a } from "./util";
import { b } from "./util";
```

#### Do

```ts
import { a, b } from "./util";
```

### TSX

#### Don't

```tsx
import { a } from "./util";
import { b } from "./util";
```

#### Do

```tsx
import { a, b } from "./util";
```
