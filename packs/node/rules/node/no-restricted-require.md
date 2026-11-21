---
id: node/no-restricted-require
title: "No require of the modules the project restricts"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "what to require instead is the project's convention, which the code does not hold"
fix_exception: value
stability: preview
options:
  modules: { type: "string[]", default: [], description: "globs of module names that may not be required; a `!`-prefixed one takes names back out" }
  messages: { type: "object", default: {}, description: "a `modules` entry to the text added to the finding" }
ported_from: [eslint-plugin-n/no-restricted-require]
since: 0.5.2
---

A module the project lists in `modules` is reported where it is `require()`d with a string (a template with no substitution counts). An entry is a glob over the module name as written (`lodash`, `lodash/*`, `@scope/**`; `*` stops at a `/`, `**` does not); a `!`-prefixed entry takes the names it matches back out of the ones listed before it, in order, so `["lodash/*", "!lodash/map"]` restricts every `lodash/…` but `lodash/map`. `messages` maps an entry to the text added to the finding. Core modules are matched as written (`fs` and `node:fs` are two names). An entry that starts with `/`, which eslint-plugin-n 18.5.0 matches against the resolved file's path, is matched against the name like any other. eslint-plugin-n's rule, read and reimplemented.

With no `modules` set the rule is silent: it reports nothing, not even that the option is missing, and the engine does not run it on a single node. It is in no preset; a project switches it on by setting the option under `[rules]` (or in a `[scope.<name>]` for some files).

In `codelaw.toml`, under `[rules]`: `"node/no-restricted-require" = { severity = "error", options = { modules = ["lodash", "lodash/*"] } }`.

**Why:** A project retires a dependency or keeps a module for one layer; the rule finds the requires that went around that.

```codelaw
native: node/no-restricted-require
```

## How to fix

1. Require what the project replaced the module with, or remove the call.

## Examples

### JavaScript

#### Don't

```js options={"modules":["lodash","lodash/*"]}
const _ = require("lodash");
```

#### Do

```js options={"modules":["lodash","lodash/*"]}
const map = require("lodash.map");
```

### JSX

#### Don't

```jsx options={"modules":["lodash","lodash/*"]}
const _ = require("lodash");
```

#### Do

```jsx options={"modules":["lodash","lodash/*"]}
const map = require("lodash.map");
```

### TypeScript

#### Don't

```ts options={"modules":["lodash","lodash/*"]}
const _ = require("lodash");
```

#### Do

```ts options={"modules":["lodash","lodash/*"]}
const map = require("lodash.map");
```

### TSX

#### Don't

```tsx options={"modules":["lodash","lodash/*"]}
const _ = require("lodash");
```

#### Do

```tsx options={"modules":["lodash","lodash/*"]}
const map = require("lodash.map");
```
