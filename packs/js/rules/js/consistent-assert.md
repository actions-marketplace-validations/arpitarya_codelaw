---
id: js/consistent-assert
title: "Call assert.ok rather than assert"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/consistent-assert]
since: 0.5.0
---

The `assert` module is called through its methods (`assert.ok(…)`), not as a function.

**Why:** `assert.ok` names the check; the bare call reads the same but hides which assertion it is.

```codelaw

match:
  kind: call_expression
  has: { field: function, kind: identifier, regex: '^assert$', pattern: $F }
where:
  $F: { imported-from: [assert, "node:assert", "assert/strict", "node:assert/strict"] }
message: "Call `assert.ok(...)` rather than `assert(...)`"
fix:
  - replace: { target: $F, with: "assert.ok" }
```

## Examples

### JavaScript

#### Don't

```js
import assert from "node:assert";
assert(ready);
```

#### Do

```js
import assert from "node:assert";
assert.ok(ready);
```

### JSX

#### Don't

```jsx
import assert from "node:assert";
assert(ready);
```

#### Do

```jsx
import assert from "node:assert";
assert.ok(ready);
```

### TypeScript

#### Don't

```ts
import assert from "node:assert";
assert(ready);
```

#### Do

```ts
import assert from "node:assert";
assert.ok(ready);
```

### TSX

#### Don't

```tsx
import assert from "node:assert";
assert(ready);
```

#### Do

```tsx
import assert from "node:assert";
assert.ok(ready);
```
