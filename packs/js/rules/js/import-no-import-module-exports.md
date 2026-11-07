---
id: js/import-no-import-module-exports
title: "No module.exports in a file that imports"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "which module system the file should use is the author's"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-import/no-import-module-exports]
since: 0.5.0
---

A file with `import` declarations does not also assign `module.exports` or `exports.x`.

**Why:** Mixing the two systems in one file breaks under bundlers that treat it as an ES module, where `module` is not defined.

```codelaw
match:
  kind: expression_statement
  any:
    - pattern: "module.exports = $X;"
    - pattern: "exports.$P = $X;"
  inside:
    kind: program
    has: { kind: import_statement }
message: "This file imports, so export with `export`"
```

## How to fix

1. Replace `module.exports = …` with `export default …` (or named exports).

## Examples

### JavaScript

#### Don't

```js
import { load } from "./load";
module.exports = load;
```

#### Do

```js
import { load } from "./load";
export default load;
```


### JSX

#### Don't

```jsx
import { load } from "./load";
module.exports = load;
```

#### Do

```jsx
import { load } from "./load";
export default load;
```


### TypeScript

#### Don't

```ts
import { load } from "./load";
module.exports = load;
```

#### Do

```ts
import { load } from "./load";
export default load;
```


### TSX

#### Don't

```tsx
import { load } from "./load";
module.exports = load;
```

#### Do

```tsx
import { load } from "./load";
export default load;
```
