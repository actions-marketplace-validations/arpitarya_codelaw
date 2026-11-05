---
id: js/no-default-import
title: "Import by name, not the default"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the module's named export for this value is not written in this file"
fix_pending: "suggest; needs cross-file"
stability: preview
ported_from: [tslint/no-default-import]
since: 0.5.0
---

An `import` names the binding it takes (`import { parse } from ...`), not the module's default export. The import-side twin of `js/no-default-export`.

**Why:** A default import can be given any local name, so the same thing is called different names across the codebase, and renaming the export doesn't reach its importers.

```codelaw
match:
  kind: import_statement
  has: { kind: import_clause, has: { kind: identifier } }
message: "Import a named export instead of the default"
```

## How to fix

1. Find the named export the module offers for this value (export it by name if it has none).
2. Import it by that name: `import { name } from "module"`.

## Examples

### JavaScript

#### Don't

```js
import parse from "./parse";
```

#### Do

```js
import { parse } from "./parse";
```

### JSX

#### Don't

```jsx
import parse from "./parse";
```

#### Do

```jsx
import { parse } from "./parse";
```

### TypeScript

#### Don't

```ts
import parse from "./parse";
```

#### Do

```ts
import { parse } from "./parse";
```

### TSX

#### Don't

```tsx
import parse from "./parse";
```

#### Do

```tsx
import { parse } from "./parse";
```
