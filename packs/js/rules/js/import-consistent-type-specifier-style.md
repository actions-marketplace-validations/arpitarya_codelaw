---
id: js/import-consistent-type-specifier-style
title: "import type rather than a type marker on every specifier"
languages: [typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-import/consistent-type-specifier-style]
since: 0.5.0
---

An import whose every named specifier is marked `type` is written as one `import type { … }` (the plugin's default, `prefer-top-level`).

**Why:** One marker at the top says the whole import goes away at compile time.

```codelaw
match:
  kind: import_statement
  not:
    any:
      - regex: '^import\s+type\b'
      - has: { kind: import_clause, has: { kind: identifier } }
      - has: { kind: import_clause, has: { kind: namespace_import } }
  has:
    kind: import_clause
    has:
      kind: named_imports
      has: { kind: import_specifier }
      not: { has: { kind: import_specifier, not: { regex: '^type\s' } } }
message: "Write `import type { … }`"
fix:
  - edit-text: { pattern: '([{,]\s*)type\s+', replace: '${1}' }
  - edit-text: { pattern: '^import\s+', replace: 'import type ' }
```

## Examples

### TypeScript

#### Don't

```ts
import { type User, type Role } from "./model";
```

#### Do

```ts
import type { User, Role } from "./model";
```


### TSX

#### Don't

```tsx
import { type User, type Role } from "./model";
```

#### Do

```tsx
import type { User, Role } from "./model";
```
