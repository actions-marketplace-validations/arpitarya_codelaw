---
id: node/no-path-concat
title: "Join paths with path.join, not +"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "the fix calls `path`, which the file may need to import"
stability: preview
ported_from: [eslint-plugin-n/no-path-concat]
since: 0.5.0
---

`__dirname` and `__filename` are joined to a path with `path.join`, not `+`. A template literal like `` `${__dirname}/x` `` is not reported.

**Why:** String concatenation hard-codes one separator and keeps `..` and doubled slashes; `path.join` writes a path for the platform the code runs on.

```codelaw
match:
  pattern: $D + $S
constraints:
  D: { regex: ^(__dirname|__filename)$ }
  S: { kind: string, regex: '^.[/\\]' }
where:
  $D: undeclared
message: "Build the path with `path.join($D, ...)`, not `+`"
fix: "path.join($D, $S)"
```

## Examples

### JavaScript

#### Don't

```js
const path = require("path");
const config = __dirname + "/config.json";
```

#### Do

```js
const path = require("path");
const config = path.join(__dirname, "/config.json");
```

### JSX

#### Don't

```jsx
const path = require("path");
const config = __dirname + "/config.json";
```

#### Do

```jsx
const path = require("path");
const config = path.join(__dirname, "/config.json");
```

### TypeScript

#### Don't

```ts
const path = require("path");
const config = __dirname + "/config.json";
```

#### Do

```ts
const path = require("path");
const config = path.join(__dirname, "/config.json");
```

### TSX

#### Don't

```tsx
const path = require("path");
const config = __dirname + "/config.json";
```

#### Do

```tsx
const path = require("path");
const config = path.join(__dirname, "/config.json");
```
