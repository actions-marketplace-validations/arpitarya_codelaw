---
id: ts/no-import-type-side-effects
title: "An import of only types is import type"
languages: [typescript, tsx]
fix: auto
stability: preview
ported_from: [typescript-eslint/no-import-type-side-effects]
since: 0.5.0
---

An import whose every named specifier is marked `type` (`import { type A } from "m"`) is written `import type { A } from "m"`. The first keeps a bare `import "m"` in the output under `verbatimModuleSyntax`; the second is removed whole. For now the rule reads an import of one specifier with no alias; a longer list waits for a native fix.

**Why:** An inline-`type`-only import still loads the module at run time, with whatever side effects it has.

```codelaw
match:
  kind: import_statement
  not: { regex: '^import\s+type\b' }
  all:
    - has:
        kind: import_clause
        not: { has: { kind: identifier } }
        has:
          kind: named_imports
          all:
            - has: { kind: import_specifier, nthChild: 1, has: { field: name, pattern: $A }, not: { has: { field: alias, regex: '[\s\S]' } } }
            - not: { has: { kind: import_specifier, not: { regex: '^type\s' } } }
            - not: { has: { nthChild: 2 } }
    - has: { field: source, pattern: $M }
message: "Every specifier is a type; write `import type`"
fix: "import type { $A } from $M;"
```

## Examples

### TypeScript

#### Don't

```ts
import { type User } from "./user";
```

#### Do

```ts
import type { User } from "./user";
```

### TSX

#### Don't

```tsx
import { type User } from "./user";
```

#### Do

```tsx
import type { User } from "./user";
```
