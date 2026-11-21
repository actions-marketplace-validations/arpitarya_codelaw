---
id: ts/no-import-type-side-effects
title: "An import of only types is import type"
languages: [typescript, tsx]
fix: auto
stability: preview
ported_from: [typescript-eslint/no-import-type-side-effects]
since: 0.5.0
---

An import whose every named specifier is marked `type` (`import { type A, type B as C } from "m"`) is written `import type { A, B as C } from "m"`. The first keeps a bare `import "m"` in the output under `verbatimModuleSyntax`; the second is removed whole. Any number of specifiers, aliased or not, is read; an import that also has a default or namespace binding, or one value specifier, is left alone.

**Why:** An inline-`type`-only import still loads the module at run time, with whatever side effects it has.

```codelaw
match:
  kind: import_statement
  not: { regex: '^import\s+type\b' }
  all:
    - has:
        kind: import_clause
        pattern: $CL
        not: { has: { kind: identifier } }
        has:
          kind: named_imports
          pattern: $NI
          all:
            - has: { kind: import_specifier }
            - not: { has: { kind: import_specifier, not: { regex: '^type\s' } } }
message: "Every specifier is a type; write `import type`"
fix:
  - insert-before: { target: $CL, text: "type " }
  - edit-text: { target: $NI, pattern: '(\{|,)(\s*)type\s+', replace: '$1$2' }
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
