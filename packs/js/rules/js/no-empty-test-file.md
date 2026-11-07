---
id: js/no-empty-test-file
title: "A test file has a test"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the tests are the author's"
fix_exception: authored
files: ["**/*.test.*", "**/*.spec.*"]
stability: preview
ported_from: [eslint-plugin-sonarjs/no-empty-test-file]
since: 0.5.0
---

A file named `*.test.*` or `*.spec.*` contains at least one `it`, `test` or `specify` call.

**Why:** An empty test file is counted as tested and checks nothing.

```codelaw
match:
  kind: program
  not:
    has:
      stopBy: end
      kind: call_expression
      has: { field: function, regex: '^(it|test|specify)\b' }
message: "This test file has no test"
```

## How to fix

1. Write the tests this file was meant to hold, or delete it.

## Examples

### JavaScript

#### Don't

```js
import { sum } from "./sum";
```

#### Do

```js
import { sum } from "./sum";
test("adds", () => {
  expect(sum(1, 2)).toBe(3);
});
```


### JSX

#### Don't

```jsx
import { sum } from "./sum";
```

#### Do

```jsx
import { sum } from "./sum";
test("adds", () => {
  expect(sum(1, 2)).toBe(3);
});
```


### TypeScript

#### Don't

```ts
import { sum } from "./sum";
```

#### Do

```ts
import { sum } from "./sum";
test("adds", () => {
  expect(sum(1, 2)).toBe(3);
});
```


### TSX

#### Don't

```tsx
import { sum } from "./sum";
```

#### Do

```tsx
import { sum } from "./sum";
test("adds", () => {
  expect(sum(1, 2)).toBe(3);
});
```
