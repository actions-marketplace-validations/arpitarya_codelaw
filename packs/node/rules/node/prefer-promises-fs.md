---
id: node/prefer-promises-fs
title: "Use the promise API of fs"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "a callback becomes an awaited promise, so the caller has to become async and handle errors with try/catch"
stability: preview
ported_from: [eslint-plugin-n/prefer-promises/fs]
since: 0.5.0
---

Code calls `fs.promises` (or `fs/promises`), not the callback form of a `fs` function. Reported: a call through the module object (`fs.readFile(...)`); a function destructured from the module is not followed.

**Why:** Callback APIs compose badly with async code: errors escape `try`/`catch`, and sequencing needs nesting.

```codelaw
match:
  kind: call_expression
  has:
    field: function
    kind: member_expression
    all:
      - has: { field: object, kind: identifier, pattern: $MOD }
      - has: { field: property, regex: ^(access|appendFile|chmod|chown|copyFile|cp|lchmod|lchown|link|lstat|lutimes|mkdir|mkdtemp|open|opendir|readdir|readFile|readlink|realpath|rename|rm|rmdir|stat|statfs|symlink|truncate|unlink|utimes|writeFile)$ }
where:
  $MOD: { imported-from: [fs, "node:fs"] }
message: "Use the promise form of `fs` (`fs/promises`)"
```

## How to fix

1. Import the function from `fs/promises` (or use `fs.promises`) and `await` it.
2. Move the callback body after the `await`, and its error branch into a `catch`.

## Examples

### JavaScript

#### Don't

```js
import fs from "node:fs";
fs.readFile(path, "utf8", (err, text) => {
  show(text);
});
```

#### Do

```js
import { readFile } from "node:fs/promises";
const text = await readFile(path, "utf8");
show(text);
```

### JSX

#### Don't

```jsx
import fs from "node:fs";
fs.readFile(path, "utf8", (err, text) => {
  show(text);
});
```

#### Do

```jsx
import { readFile } from "node:fs/promises";
const text = await readFile(path, "utf8");
show(text);
```

### TypeScript

#### Don't

```ts
import fs from "node:fs";
fs.readFile(path, "utf8", (err, text) => {
  show(text);
});
```

#### Do

```ts
import { readFile } from "node:fs/promises";
const text = await readFile(path, "utf8");
show(text);
```

### TSX

#### Don't

```tsx
import fs from "node:fs";
fs.readFile(path, "utf8", (err, text) => {
  show(text);
});
```

#### Do

```tsx
import { readFile } from "node:fs/promises";
const text = await readFile(path, "utf8");
show(text);
```
