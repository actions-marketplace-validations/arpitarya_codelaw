---
id: js/import-no-internal-modules
title: "Imports stop at a module's entry point"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the module has to export the name from its entry point first, which is another file"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-import/no-internal-modules]
since: 0.5.0
---

An import names a package or a folder (`lodash`, `./widgets`), not a file inside one (`lodash/fp/map`, `./widgets/button/style`). The path is read as written, with `./` and `../` steps folded; upstream also skips a path its resolver cannot find, which this rule does not check.

**Why:** Reaching inside a module couples the importer to its internal layout, so the module cannot reorganise its files without breaking callers.

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
  has:
    kind: string_fragment
    pattern: $P
    regex: '^(\.{1,2}/)*(@[^/]+/)?[^@./][^/]*/[^/]+'
message: "`$P` reaches inside another module; import it from that module's entry point"
```

## How to fix

1. Export what you need from the module's entry point (its `index` file, or `exports` in its `package.json`).
2. Import it from there: `./widgets` rather than `./widgets/button/style`.

## Examples

### JavaScript

#### Don't

```js
import { Button } from "./widgets/button/Button";
use(Button);
```

#### Do

```js
import { Button } from "./widgets";
use(Button);
```

### JSX

#### Don't

```jsx
import { Button } from "./widgets/button/Button";
use(Button);
```

#### Do

```jsx
import { Button } from "./widgets";
use(Button);
```

### TypeScript

#### Don't

```ts
import { Button } from "./widgets/button/Button";
use(Button);
```

#### Do

```ts
import { Button } from "./widgets";
use(Button);
```

### TSX

#### Don't

```tsx
import { Button } from "./widgets/button/Button";
use(Button);
```

#### Do

```tsx
import { Button } from "./widgets";
use(Button);
```
