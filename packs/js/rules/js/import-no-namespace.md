---
id: js/import-no-namespace
title: "No namespace import"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "which names are used is read through the file; the author lists them"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-import/no-namespace, eslint-plugin-sonarjs/no-wildcard-import]
since: 0.5.0
---

A module is imported by the names used (`import { a, b } from "m"`), not as a namespace (`import * as m from "m"`).

**Why:** Named imports say what the file depends on and let bundlers drop the rest.

```codelaw
match:
  kind: import_statement
  has: { stopBy: end, kind: namespace_import }
message: "Import the names you use instead of the whole namespace"
```

## How to fix

1. List the names the file uses from the module in braces, and drop the namespace prefix at each use.

## Examples

### JavaScript

#### Don't

```js
import * as path from "node:path";
const file = path.join(dir, name);
```

#### Do

```js
import { join } from "node:path";
const file = join(dir, name);
```


### JSX

#### Don't

```jsx
import * as path from "node:path";
const file = path.join(dir, name);
```

#### Do

```jsx
import { join } from "node:path";
const file = join(dir, name);
```


### TypeScript

#### Don't

```ts
import * as path from "node:path";
const file = path.join(dir, name);
```

#### Do

```ts
import { join } from "node:path";
const file = join(dir, name);
```


### TSX

#### Don't

```tsx
import * as path from "node:path";
const file = path.join(dir, name);
```

#### Do

```tsx
import { join } from "node:path";
const file = join(dir, name);
```
