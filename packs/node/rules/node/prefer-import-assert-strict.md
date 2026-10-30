---
id: node/prefer-import-assert-strict
title: "Import assert/strict"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "in strict mode `assert.equal` and `assert.deepEqual` compare with `===` and stricter deep equality, so a passing check can start failing"
stability: preview
ported_from: [eslint-plugin-n/prefer-import/assert-strict]
since: 0.5.0
---

Code imports `assert/strict` rather than the legacy `assert` module.

**Why:** The legacy `assert.equal` compares with `==`, so `assert.equal(1, "1")` passes; the strict module's assertions don't coerce.

```codelaw
match:
  kind: string_fragment
  regex: ^(node:)?assert$
  inside:
    kind: string
    any:
      - inside: { kind: import_statement, field: source }
      - inside: { kind: export_statement, field: source }
      - all:
          - nthChild: 1
          - inside: { kind: arguments, inside: { kind: call_expression, has: { field: function, regex: ^(require|import)$ } } }
message: "Import `$$MATCH/strict`; the legacy assertions compare loosely"
fix: "$$MATCH/strict"
```

## Examples

### JavaScript

#### Don't

```js
import assert from "node:assert";
assert.equal(total, 3);
```

#### Do

```js
import assert from "node:assert/strict";
assert.equal(total, 3);
```

### JSX

#### Don't

```jsx
import assert from "node:assert";
assert.equal(total, 3);
```

#### Do

```jsx
import assert from "node:assert/strict";
assert.equal(total, 3);
```

### TypeScript

#### Don't

```ts
import assert from "node:assert";
assert.equal(total, 3);
```

#### Do

```ts
import assert from "node:assert/strict";
assert.equal(total, 3);
```

### TSX

#### Don't

```tsx
import assert from "node:assert";
assert.equal(total, 3);
```

#### Do

```tsx
import assert from "node:assert/strict";
assert.equal(total, 3);
```
