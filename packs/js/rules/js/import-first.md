---
id: js/import-first
title: "Imports come before other statements"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-import/first]
since: 0.5.0
---

Every `import` sits at the top of the module, before any other statement (directives such as `"use strict"` and comments may come first).

**Why:** Imports run before the module body wherever they are written, so an import placed after code misleads the reader about what runs first.

```codelaw
match:
  kind: import_statement
  inside: { kind: program }
  follows:
    stopBy: end
    pattern: $FIRST
    not:
      any:
        - kind: import_statement
        - kind: comment
        - kind: hash_bang_line
        - kind: expression_statement
          has: { kind: string, nthChild: 1 }
          not: { has: { nthChild: 2 } }
message: "This import comes after other code; move it up with the other imports"
fix:
  - remove
  - insert-before: { target: $FIRST, text: "$$MATCH\n" }
```

## Examples

### JavaScript

#### Don't

```js
import { a } from "./a";
a();
import { b } from "./b";
b();
```

#### Do

```js
import { a } from "./a";
import { b } from "./b";
a();
b();
```

### JSX

#### Don't

```jsx
import { a } from "./a";
a();
import { b } from "./b";
b();
```

#### Do

```jsx
import { a } from "./a";
import { b } from "./b";
a();
b();
```

### TypeScript

#### Don't

```ts
import { a } from "./a";
a();
import { b } from "./b";
b();
```

#### Do

```ts
import { a } from "./a";
import { b } from "./b";
a();
b();
```

### TSX

#### Don't

```tsx
import { a } from "./a";
a();
import { b } from "./b";
b();
```

#### Do

```tsx
import { a } from "./a";
import { b } from "./b";
a();
b();
```
