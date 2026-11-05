---
id: js/import-no-relative-parent-imports
title: "No imports from a parent folder"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the module has to move, or be published as a package, which reaches beyond this file"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-import/no-relative-parent-imports]
since: 0.5.0
---

An `import`, `export … from` or `import()` path does not climb to a parent folder (`../`); modules depend on what sits beside or below them, or on packages.

**Why:** Imports that reach up the tree tie a folder to its surroundings, so it cannot be moved or extracted, and the dependency graph stops following the folder structure.

```codelaw
utils:
  source:
    kind: string
    any:
      - inside: { kind: import_statement, field: source }
      - inside: { kind: export_statement, field: source }
      - inside:
          kind: arguments
          inside:
            kind: call_expression
            has: { field: function, kind: import }
match:
  matches: source
  has: { kind: string_fragment, regex: '^\.\.(/|$)', pattern: $P }
message: "`$P` reaches into a parent folder; pass the dependency in, move the module down, or import it as a package"
```

## How to fix

1. Decide where the shared code belongs: move the imported module into this folder (or below it), or move this module up beside it.
2. If several folders share it, make it a package of the workspace and import it by name, or pass it in as an argument.

## Examples

### JavaScript

#### Don't

```js
import { format } from "../utils/format";
format(1);
```

#### Do

```js
import { format } from "./format";
format(1);
```

### JSX

#### Don't

```jsx
import { format } from "../utils/format";
format(1);
```

#### Do

```jsx
import { format } from "./format";
format(1);
```

### TypeScript

#### Don't

```ts
import { format } from "../utils/format";
format(1);
```

#### Do

```ts
import { format } from "./format";
format(1);
```

### TSX

#### Don't

```tsx
import { format } from "../utils/format";
format(1);
```

#### Do

```tsx
import { format } from "./format";
format(1);
```
