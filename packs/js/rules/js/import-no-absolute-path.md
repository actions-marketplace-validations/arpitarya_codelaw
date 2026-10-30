---
id: js/import-no-absolute-path
title: "Imports use relative paths, not absolute ones"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the relative path depends on where this file sits, which the rule does not see"
stability: preview
ported_from: [eslint-plugin-import/no-absolute-path]
since: 0.5.0
---

An `import`, `export … from`, `import()` or `require()` names a module by a package name or a path relative to the file, never by an absolute file-system path.

**Why:** An absolute path works only on the machine where it was written; on any other checkout the module is not found.

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
            has: { field: function, regex: ^(require|import)$ }
match:
  matches: source
  has: { kind: string_fragment, regex: ^/, pattern: $P }
message: "`$P` is an absolute path; name the module by a path relative to this file"
```

## How to fix

1. Work out the path from this file to the module (`../lib/util` rather than `/home/me/app/lib/util`).
2. Write that relative path, or the package name when the module is a dependency.

## Examples

### JavaScript

#### Don't

```js
import { helper } from "/home/me/app/lib/helper";
helper();
```

#### Do

```js
import { helper } from "../lib/helper";
helper();
```

### JSX

#### Don't

```jsx
import { helper } from "/home/me/app/lib/helper";
helper();
```

#### Do

```jsx
import { helper } from "../lib/helper";
helper();
```

### TypeScript

#### Don't

```ts
import { helper } from "/home/me/app/lib/helper";
helper();
```

#### Do

```ts
import { helper } from "../lib/helper";
helper();
```

### TSX

#### Don't

```tsx
import { helper } from "/home/me/app/lib/helper";
helper();
```

#### Do

```tsx
import { helper } from "../lib/helper";
helper();
```
