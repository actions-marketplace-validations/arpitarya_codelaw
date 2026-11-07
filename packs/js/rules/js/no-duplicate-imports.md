---
id: js/no-duplicate-imports
title: "One import per module"
languages: [javascript, jsx, typescript, tsx]
fix: auto
fix_reason: "two imports of `{ … }` names only are merged automatically, a name both hold kept once; an import with a default or namespace binding is merged by hand, since the two can't always share one statement"
stability: preview
ported_from: [eslint/no-duplicate-imports, eslint-plugin-import/no-duplicates]
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
  any:
    - all:
        - regex: '^import\s*\{'
        - pattern: $DUP
        - has: { kind: import_clause, has: { kind: named_imports, pattern: $FROM } }
        - follows:
            stopBy: end
            kind: import_statement
            regex: '^import\s*\{'
            all:
              - has: { field: source, pattern: $S }
              - has: { kind: import_clause, has: { kind: named_imports, pattern: $INTO } }
    - regex: ''
message: "`$S` is imported again; merge the two imports"
fix:
  - merge-into: { target: $FROM, into: $INTO, unique: true }
  - remove: $DUP
```

## How to fix

1. Move the second import's names into the first, and delete the second.

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
