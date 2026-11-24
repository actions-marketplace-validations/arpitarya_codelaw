---
id: js/import-no-restricted-paths
title: "No import across a restricted zone"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "moving the code or the import to the right side of the boundary is a design change"
fix_exception: restructure
stability: preview
options:
  zones: { type: "object", default: {}, description: "named zones, each `{ target, from, except, message }`: files under `target` may not import what is under `from`, except the `except` paths" }
  basePath: { type: "string", default: "", description: "the folder `target`, `from` and `except` are relative to (default: the project root)" }
ported_from: [eslint-plugin-import/no-restricted-paths]
since: 0.5.2
---

A file under a zone's `target` that imports a file under its `from` is reported: `zones = { client = { target = "client", from = "server" } }` keeps client code from importing server code. `target`, `from` and `except` are each a folder (everything under it), a glob, or a list of them; `except` is relative to a folder `from` and a glob of its own beside a glob `from`; `message` is added to the finding. The import is resolved the way a bundler resolves a relative one: the path as written, then with `.js`, `.jsx`, `.ts`, `.tsx`, `.mjs`, `.cjs`, `.mts`, `.cts` or `.json`, then as a folder's `index`. `import`, `export … from`, `require()` and `import()` of a string are read. A specifier that is not relative (`react`, `@scope/pkg`) is not resolved, since the rule reads the files of the project and not `node_modules`; a path alias is not followed either. eslint-plugin-import 2.32.0's `no-restricted-paths`, read and reimplemented: upstream's `zones` is a list, here named, so a project's presets can add or replace one by name.

With no `zones` set the rule is silent: it reports nothing, not even that the option is missing, and the engine does not run it on a single node. It is in no preset; a project switches it on by setting the option under `[rules]` (or in a `[scope.<name>]` for some files).

In `codelaw.toml`, under `[rules]`: `"js/import-no-restricted-paths" = { severity = "error", options = { zones = { client = { target = "client", from = "server" } } } }`.

**Why:** Code for one environment (client, server, a layer) must not reach into another; a boundary only the rule enforces is a boundary that holds.

```codelaw
native: js/import-no-restricted-paths
```

## How to fix

1. Move what both sides need to a shared folder, or move the importing file to the other side.

## Examples

### JavaScript

#### Don't

```js options={"zones":{"client":{"target":"client","from":"server"}}}
// client/app.js
import { secret } from "../server/secret";
```

#### Do

```js options={"zones":{"client":{"target":"client","from":"server"}}}
// client/app.js
import { api } from "../shared/api";
```

### JSX

#### Don't

```jsx options={"zones":{"client":{"target":"client","from":"server"}}}
// client/app.js
import { secret } from "../server/secret";
```

#### Do

```jsx options={"zones":{"client":{"target":"client","from":"server"}}}
// client/app.js
import { api } from "../shared/api";
```

### TypeScript

#### Don't

```ts options={"zones":{"client":{"target":"client","from":"server"}}}
// client/app.js
import { secret } from "../server/secret";
```

#### Do

```ts options={"zones":{"client":{"target":"client","from":"server"}}}
// client/app.js
import { api } from "../shared/api";
```

### TSX

#### Don't

```tsx options={"zones":{"client":{"target":"client","from":"server"}}}
// client/app.js
import { secret } from "../server/secret";
```

#### Do

```tsx options={"zones":{"client":{"target":"client","from":"server"}}}
// client/app.js
import { api } from "../shared/api";
```
