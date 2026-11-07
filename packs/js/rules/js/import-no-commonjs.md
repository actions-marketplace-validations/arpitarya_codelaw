---
id: js/import-no-commonjs
title: "No CommonJS require or exports"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "moving to ES modules changes how the file is loaded; the author converts it"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-import/no-commonjs]
since: 0.5.0
---

A module uses `import` and `export`, not `require()` at its top level, `module.exports` or `exports.x`. A `require` inside a function or a condition is left alone, as upstream's default allows.

**Why:** ES modules are statically analysable, and one module system per project avoids interop surprises.

```codelaw
match:
  any:
    - kind: expression_statement
      any:
        - pattern: "module.exports = $X;"
        - pattern: "exports.$P = $X;"
    - kind: lexical_declaration
      pattern: "const $A = require($S);"
      inside: { kind: program }
message: "Use `import`/`export` instead of CommonJS"
```

## How to fix

1. Replace `require` with `import` and `module.exports`/`exports.x` with `export`, and set the package's module type.

## Examples

### JavaScript

#### Don't

```js
const fs = require("node:fs");
module.exports = load;
```

#### Do

```js
import fs from "node:fs";
export default load;
```


### JSX

#### Don't

```jsx
const fs = require("node:fs");
module.exports = load;
```

#### Do

```jsx
import fs from "node:fs";
export default load;
```


### TypeScript

#### Don't

```ts
const fs = require("node:fs");
module.exports = load;
```

#### Do

```ts
import fs from "node:fs";
export default load;
```


### TSX

#### Don't

```tsx
const fs = require("node:fs");
module.exports = load;
```

#### Do

```tsx
import fs from "node:fs";
export default load;
```
