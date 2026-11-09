---
id: testing/prefer-importing-jest-globals
title: "Jest's globals are imported from `@jest/globals`"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: auto
stability: preview
options: { types: { type: "string[]", default: ["hook", "describe", "test", "expect", "jest"] } }
ported_from: ["eslint-plugin-jest/prefer-importing-jest-globals"]
since: 0.5.0
---

A test file imports every Jest global it uses (`describe`, `it`, `test`, `expect`, `jest`, the hooks) from `@jest/globals` instead of relying on the runner's globals. One finding per file, on the first global; the fix adds one import with every global the file uses, or merges them into an existing `@jest/globals` import.

**Why:** An imported name is checked by the editor and the type checker, and a test file that imports its runner runs the same under another runner's globals. The import is also the one place a file says which tool it was written for.

```codelaw
native: testing/prefer-importing-jest-globals
```

`types` limits which kinds of global count: `hook`, `describe`, `test`, `expect`, `jest` (all five by default).


## Examples

### JavaScript

#### Don't

```js
describe("sum", () => {
  it("adds", () => {
    expect(1 + 1).toBe(2);
  });
});
```

#### Do

```js
import { describe, expect, it } from '@jest/globals';
describe("sum", () => {
  it("adds", () => {
    expect(1 + 1).toBe(2);
  });
});
```

### JSX

#### Don't

```jsx
describe("sum", () => {
  it("adds", () => {
    expect(1 + 1).toBe(2);
  });
});
```

#### Do

```jsx
import { describe, expect, it } from '@jest/globals';
describe("sum", () => {
  it("adds", () => {
    expect(1 + 1).toBe(2);
  });
});
```

### TypeScript

#### Don't

```ts
describe("sum", () => {
  it("adds", () => {
    expect(1 + 1).toBe(2);
  });
});
```

#### Do

```ts
import { describe, expect, it } from '@jest/globals';
describe("sum", () => {
  it("adds", () => {
    expect(1 + 1).toBe(2);
  });
});
```

### TSX

#### Don't

```tsx
describe("sum", () => {
  it("adds", () => {
    expect(1 + 1).toBe(2);
  });
});
```

#### Do

```tsx
import { describe, expect, it } from '@jest/globals';
describe("sum", () => {
  it("adds", () => {
    expect(1 + 1).toBe(2);
  });
});
```

## Differences from the source

- A file with no `import`, `export` or `require` is a module, so the fix is an `import`; a file that calls `require(…)` or sets `module.exports`, or ends in `.cjs`/`.cts`, gets `const { … } = require('@jest/globals');`. Upstream asks the parser's source type, which codelaw does not have.
- `import type { … } from '@jest/globals'` and an import with a namespace (`import * as jest`) are not merged into: a new import line is added.
- A `settings.jest.globalAliases` and `globalPackage` setting are not read.
