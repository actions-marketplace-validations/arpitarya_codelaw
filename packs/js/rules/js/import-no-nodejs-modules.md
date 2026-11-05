---
id: js/import-no-nodejs-modules
title: "No Node.js built-in modules"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "what replaces the built-in (a browser API, a package, an injected dependency) is a design choice"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-import/no-nodejs-modules]
since: 0.5.0
---

Code meant to run outside Node.js (a browser bundle, an edge function) does not import Node.js built-in modules such as `fs`, `path` or `node:crypto`.

**Why:** A built-in module does not exist in the browser: the bundle fails to build, or ships a polyfill nobody asked for.

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
    regex: '^(node:[a-z_/]+|(assert|async_hooks|buffer|child_process|cluster|console|constants|crypto|dgram|diagnostics_channel|dns|domain|events|fs|http|http2|https|inspector|module|net|os|path|perf_hooks|process|punycode|querystring|readline|repl|stream|string_decoder|sys|timers|tls|trace_events|tty|url|util|v8|vm|wasi|worker_threads|zlib)(/[a-z_/]+)?)$'
message: "`$P` is a Node.js built-in module; use a browser API or a package instead"
```

## How to fix

1. Find what the module is used for here (`path.join`, `crypto.randomUUID`, `events.EventEmitter`).
2. Replace it with the platform API (`crypto.randomUUID()` on `globalThis`, `URL`, `EventTarget`) or a package built for the browser, or move the code into a Node-only module.

## Examples

### JavaScript

#### Don't

```js
import { join } from "path";
join("a", "b");
```

#### Do

```js
const joined = ["a", "b"].join("/");
use(joined);
```

### JSX

#### Don't

```jsx
import { join } from "path";
join("a", "b");
```

#### Do

```jsx
const joined = ["a", "b"].join("/");
use(joined);
```

### TypeScript

#### Don't

```ts
import { join } from "path";
join("a", "b");
```

#### Do

```ts
const joined = ["a", "b"].join("/");
use(joined);
```

### TSX

#### Don't

```tsx
import { join } from "path";
join("a", "b");
```

#### Do

```tsx
const joined = ["a", "b"].join("/");
use(joined);
```
