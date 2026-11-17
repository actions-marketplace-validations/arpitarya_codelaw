---
id: js/use-node-assert-strict
title: "Import `node:assert/strict`"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "`assert/strict` compares with `===` and deep-strict equality where `assert` uses `==`, so a test that relied on the loose forms can start to fail"
stability: preview
ported_from: [biome/style/useNodeAssertStrict]
since: 0.5.1
---

`assert` is imported from `node:assert/strict`, not from `assert` or `node:assert`. The suggestion rewrites the module name and keeps the quotes. Only `import` statements are read. Behaviour of Biome's `useNodeAssertStrict`.

**Why:** The strict module makes `assert.equal` mean `===` and `assert.deepEqual` mean `deepStrictEqual`, so a test cannot pass on a loose comparison by accident.

```codelaw
match:
  kind: import_statement
  has: { field: source, kind: string, regex: '^["'']((node:)?assert)["'']$', pattern: $S }
message: "Import `node:assert/strict` instead of `assert`"
fix:
  - edit-text: { target: $S, pattern: '^(.)(node:)?assert(.)$', replace: '${1}node:assert/strict${3}' }
```

## Examples

### JavaScript

#### Don't

```js
import assert from "node:assert";
```

#### Do

```js
import assert from "node:assert/strict";
```

### JSX

#### Don't

```jsx
import assert from "node:assert";
```

#### Do

```jsx
import assert from "node:assert/strict";
```

### TypeScript

#### Don't

```ts
import assert from "node:assert";
```

#### Do

```ts
import assert from "node:assert/strict";
```

### TSX

#### Don't

```tsx
import assert from "node:assert";
```

#### Do

```tsx
import assert from "node:assert/strict";
```
