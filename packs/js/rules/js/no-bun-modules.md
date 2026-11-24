---
id: js/no-bun-modules
title: "No Bun-specific module"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the Node.js or web-standard replacement depends on which `bun` API the code uses"
fix_exception: restructure
stability: preview
ported_from: [biome/nursery/noBunModules]
since: 0.5.1
---

Nothing is imported or required from `bun` or `bun:*` (`bun:test`, `bun:sqlite`, `bun:ffi`). Behaviour of Biome's `noBunModules`, as `import`, `export … from`, `require()` and `import()` of the module name.

**Why:** A `bun:` module exists only in the Bun runtime; code that imports one cannot run on Node.js, Deno or the browser.

```codelaw
match:
  any:
    - kind: import_statement
      has: { field: source, kind: string, regex: '^["'']bun(:[^"'']*)?["'']$' }
    - kind: export_statement
      has: { field: source, kind: string, regex: '^["'']bun(:[^"'']*)?["'']$' }
    - kind: call_expression
      all:
        - has: { field: function, regex: '^(require|import)$' }
        - has: { field: arguments, has: { kind: string, regex: '^["'']bun(:[^"'']*)?["'']$' } }
message: "Do not use a Bun-specific module"
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JavaScript

#### Don't

```js
import { Database } from "bun:sqlite";
```

#### Do

```js
import { DatabaseSync } from "node:sqlite";
```

### JSX

#### Don't

```jsx
import { Database } from "bun:sqlite";
```

#### Do

```jsx
import { DatabaseSync } from "node:sqlite";
```

### TypeScript

#### Don't

```ts
import { Database } from "bun:sqlite";
```

#### Do

```ts
import { DatabaseSync } from "node:sqlite";
```

### TSX

#### Don't

```tsx
import { Database } from "bun:sqlite";
```

#### Do

```tsx
import { DatabaseSync } from "node:sqlite";
```
