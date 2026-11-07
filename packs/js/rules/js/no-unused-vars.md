---
id: js/no-unused-vars
title: "Every declared name is used"
languages: [javascript, jsx]
fix: auto
fix_reason: "auto for a name nothing reads or writes: an import with another binding left, a variable (an initialiser that could do something kept as a statement), a function, an interface or type alias, a catch binding; removing an import's last binding drops the module's load and is suggest; a parameter, a destructured name or a class is manual"
fix_exception: restructure
stability: preview
options:
  vars: { type: string, default: all, description: "all, or local to skip module-level names" }
  args: { type: string, default: after-used, description: "after-used, all or none" }
  caughtErrors: { type: string, default: all, description: "all or none" }
  ignoreRestSiblings: { type: boolean, default: false }
  varsIgnorePattern: { type: string, description: "a regular expression of names to skip" }
  argsIgnorePattern: { type: string }
  caughtErrorsIgnorePattern: { type: string }
  destructuredArrayIgnorePattern: { type: string }
ported_from: [eslint/no-unused-vars, eslint-plugin-sonarjs/no-unused-function-argument]
since: 0.1.0
---

Every variable, function, class, import, parameter and caught error is read somewhere. Parameters
before the last used one are allowed (`args: after-used`).

**Why:** An unused declaration is either dead code or a sign the code doesn't do what it was meant
to.

```codelaw
native: js/no-unused-vars
```

## How to fix

1. Check the initialiser has no side effect that matters.
2. Remove the declaration, or use it where it was meant to be used.
3. For a parameter that must stay for its position, prefix it with `_` and set `argsIgnorePattern: "^_"`.

## Examples

### JavaScript

#### Don't

```js
import { readFile, writeFile } from "node:fs";
export const load = (path) => readFile(path);
```

#### Do

```js
import { readFile } from "node:fs";
export const load = (path) => readFile(path);
```

### JSX

#### Don't

```jsx
import { readFile, writeFile } from "node:fs";
export const load = (path) => readFile(path);
```

#### Do

```jsx
import { readFile } from "node:fs";
export const load = (path) => readFile(path);
```
