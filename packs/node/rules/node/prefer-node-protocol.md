---
id: node/prefer-node-protocol
title: "Import built-in modules with node:"
languages: [javascript, jsx, typescript, tsx]
fix: auto
fix_reason: "Node resolves `fs` and `node:fs` to the same built-in module"
stability: preview
ported_from: [eslint-plugin-n/prefer-node-protocol]
since: 0.5.0
---

An import or `require()` of a Node built-in names it with the `node:` prefix. `punycode` is left out: code that names it almost always means the npm package.

**Why:** `node:fs` can't be shadowed by a package in `node_modules`, and tells the reader at a glance that the module is Node's own.

```codelaw
match:
  kind: string_fragment
  regex: ^(assert|assert/strict|async_hooks|buffer|child_process|cluster|console|constants|crypto|dgram|diagnostics_channel|dns|dns/promises|domain|events|fs|fs/promises|http|http2|https|inspector|inspector/promises|module|net|os|path|path/posix|path/win32|perf_hooks|process|querystring|readline|readline/promises|repl|stream|stream/consumers|stream/promises|stream/web|string_decoder|sys|timers|timers/promises|tls|trace_events|tty|url|util|util/types|v8|vm|wasi|worker_threads|zlib)$
  inside:
    kind: string
    any:
      - inside: { kind: import_statement, field: source }
      - inside: { kind: export_statement, field: source }
      - all:
          - nthChild: 1
          - inside: { kind: arguments, inside: { kind: call_expression, has: { field: function, regex: ^(require|import)$ } } }
message: "Import the built-in as `node:$$MATCH`"
fix: "node:$$MATCH"
```

## Examples

### JavaScript

#### Don't

```js
import { readFile } from "fs/promises";
const path = require("path");
```

#### Do

```js
import { readFile } from "node:fs/promises";
const path = require("node:path");
```

### JSX

#### Don't

```jsx
import { readFile } from "fs/promises";
const path = require("path");
```

#### Do

```jsx
import { readFile } from "node:fs/promises";
const path = require("node:path");
```

### TypeScript

#### Don't

```ts
import { readFile } from "fs/promises";
const path = require("path");
```

#### Do

```ts
import { readFile } from "node:fs/promises";
const path = require("node:path");
```

### TSX

#### Don't

```tsx
import { readFile } from "fs/promises";
const path = require("path");
```

#### Do

```tsx
import { readFile } from "node:fs/promises";
const path = require("node:path");
```
