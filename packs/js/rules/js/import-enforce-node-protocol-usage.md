---
id: js/import-enforce-node-protocol-usage
title: "Node.js built-ins are imported with node:"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-import/enforce-node-protocol-usage]
since: 0.5.0
---

A Node.js built-in module is imported by its `node:` name (`node:fs`, `node:path`), in `import`, `export … from`, `import()` and `require()`. This is upstream's `always` setting; the rule takes no option.

**Why:** The prefix says at a glance that the module is part of Node.js, and no package of the same name in `node_modules` can stand in for it.

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
  kind: string_fragment
  pattern: $P
  inside: { matches: source }
  regex: '^(assert|async_hooks|buffer|child_process|cluster|console|constants|crypto|dgram|diagnostics_channel|dns|domain|events|fs|http|http2|https|inspector|module|net|os|path|perf_hooks|process|punycode|querystring|readline|repl|stream|string_decoder|sys|timers|tls|trace_events|tty|url|util|v8|vm|wasi|worker_threads|zlib)(/(promises|posix|win32|web|consumers|types|strict))?$'
message: "`$P` is a Node.js built-in; import it as `node:$P`"
fix: "node:$P"
```

## Examples

### JavaScript

#### Don't

```js
import { readFile } from "fs/promises";
const path = require("path");
readFile(path.join("a", "b"));
```

#### Do

```js
import { readFile } from "node:fs/promises";
const path = require("node:path");
readFile(path.join("a", "b"));
```

### JSX

#### Don't

```jsx
import { readFile } from "fs/promises";
const path = require("path");
readFile(path.join("a", "b"));
```

#### Do

```jsx
import { readFile } from "node:fs/promises";
const path = require("node:path");
readFile(path.join("a", "b"));
```

### TypeScript

#### Don't

```ts
import { readFile } from "fs/promises";
const path = require("path");
readFile(path.join("a", "b"));
```

#### Do

```ts
import { readFile } from "node:fs/promises";
const path = require("node:path");
readFile(path.join("a", "b"));
```

### TSX

#### Don't

```tsx
import { readFile } from "fs/promises";
const path = require("path");
readFile(path.join("a", "b"));
```

#### Do

```tsx
import { readFile } from "node:fs/promises";
const path = require("node:path");
readFile(path.join("a", "b"));
```
