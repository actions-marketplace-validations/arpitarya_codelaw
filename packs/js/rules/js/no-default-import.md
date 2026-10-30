---
id: js/no-default-import
title: "No default imports"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "value: the named export to import is the module's, which the importing file does not show"
stability: preview
ported_from: [tslint/no-default-import]
since: 0.5.0
---

Modules are imported by name, never by their default export: `import Button from "./button"` and `import { default as Button } from …` are both reported. Namespace imports and side-effect imports are fine.

**Why:** A default import can be named anything at each import site, so one thing ends up with several names; the import-side twin of `js/no-default-export`.

```codelaw
match:
  kind: import_statement
  has:
    kind: import_clause
    any:
      - has: { kind: identifier }
      - has: { kind: named_imports, has: { kind: import_specifier, has: { field: name, regex: '^default$' } } }
message: "Import this module's named export instead of its default"
```

## How to fix

1. Import the name the module exports: `import { Button } from "./button"` (export it by name there if it is not).

## Examples

### JavaScript

#### Don't

```js
import Button from "./button";
```

#### Do

```js
import { Button } from "./button";
```

### JSX

#### Don't

```jsx
import Button from "./button";
```

#### Do

```jsx
import { Button } from "./button";
```

### TypeScript

#### Don't

```ts
import Button from "./button";
```

#### Do

```ts
import { Button } from "./button";
```

### TSX

#### Don't

```tsx
import Button from "./button";
```

#### Do

```tsx
import { Button } from "./button";
```
