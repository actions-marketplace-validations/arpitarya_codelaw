---
id: js/no-unused-imports
title: "No unused imports"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "removing an import can drop a module's side effects only when the whole statement goes; review before accepting"
stability: preview
ported_from: [biome/correctness/noUnusedImports]
since: 0.5.0
---

Every name an `import` brings in is used in the file.

**Why:** An unused import is dead code that still loads the module, and hides what the file really depends on.

```codelaw
match:
  any:
    - kind: import_specifier
      not: { has: { field: alias, regex: '.' } }
      has: { field: name, pattern: $N }
    - kind: import_specifier
      has: { field: alias, pattern: $N }
    - kind: identifier
      pattern: $N
      inside: { kind: import_clause }
    - kind: namespace_import
      has: { kind: identifier, pattern: $N }
where:
  $N: unused
message: "`$N` is imported but never used"
fix:
  - remove
```

## Examples

### JavaScript

#### Don't

```js
import { readFile, writeFile } from 'node:fs';
writeFile('a', 'b');
```

#### Do

```js
import { writeFile } from 'node:fs';
writeFile('a', 'b');
```

### JSX

#### Don't

```jsx
import { readFile, writeFile } from 'node:fs';
writeFile('a', 'b');
```

#### Do

```jsx
import { writeFile } from 'node:fs';
writeFile('a', 'b');
```

### TypeScript

#### Don't

```ts
import { readFile, writeFile } from 'node:fs';
writeFile('a', 'b');
```

#### Do

```ts
import { writeFile } from 'node:fs';
writeFile('a', 'b');
```

### TSX

#### Don't

```tsx
import { readFile, writeFile } from 'node:fs';
writeFile('a', 'b');
```

#### Do

```tsx
import { writeFile } from 'node:fs';
writeFile('a', 'b');
```

## Differences from the source

A default import or namespace import is reported on its name, and the fix removes only that name.
