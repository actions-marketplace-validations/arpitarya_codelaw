---
id: js/no-redundant-use-strict
title: "No redundant \"use strict\""
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [biome/suspicious/noRedundantUseStrict]
since: 0.5.0
---

A `"use strict"` directive is not written where the code is already strict: in a module, in a class, or inside a function in a file that is strict already.

**Why:** Modules and classes are always strict; the directive there is noise that suggests the code around it is not.

```codelaw
utils:
  directive:
    kind: expression_statement
    regex: '^["'']use strict["''];?$'
  strict_file:
    kind: program
    any:
      - has: { any: [{kind: import_statement}, {kind: export_statement}] }
      - has: { kind: expression_statement, regex: '^["'']use strict["''];?$', nthChild: 1 }
match:
  matches: directive
  any:
    - inside: { kind: class_body, stopBy: end }
    - inside: { kind: program, has: { any: [{kind: import_statement}, {kind: export_statement}] } }
    - inside: { kind: statement_block, inside: { stopBy: end, matches: strict_file } }
message: "This code is already strict; remove the directive"
fix:
  - remove
```

## Examples

### JavaScript

#### Don't

```js
import { run } from './run';
'use strict';
run();
```

#### Do

```js
import { run } from './run';
run();
```

#### Fixed

```js
import { run } from './run';
run();
```

### JSX

#### Don't

```jsx
import { run } from './run';
'use strict';
run();
```

#### Do

```jsx
import { run } from './run';
run();
```

#### Fixed

```jsx
import { run } from './run';
run();
```

### TypeScript

#### Don't

```ts
import { run } from './run';
'use strict';
run();
```

#### Do

```ts
import { run } from './run';
run();
```

#### Fixed

```ts
import { run } from './run';
run();
```

### TSX

#### Don't

```tsx
import { run } from './run';
'use strict';
run();
```

#### Do

```tsx
import { run } from './run';
run();
```

#### Fixed

```tsx
import { run } from './run';
run();
```
