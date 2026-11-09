---
id: testing/prefer-importing-vitest-globals
title: "Vitest's globals are imported from `vitest`"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: auto
stability: preview
ported_from: ["@vitest/eslint-plugin/prefer-importing-vitest-globals"]
since: 0.5.0
---

A test file that calls one of Vitest's globals (`describe`, `it`, `test`, `suite`, `expect`, `assert`, the hooks, `expectTypeOf`, `assertType`, `onTestFailed`, `onTestFinished`, `vi`, `vitest`, `chai`) as a function imports it from `vitest`. Opt-in, the opposite of `testing/no-importing-vitest-globals`: a project sets one of the two.

**Why:** With `globals: false` (Vitest's default) the names are not defined at run time, and an import makes the dependency visible to the editor and the type checker.

```codelaw
native: testing/prefer-importing-vitest-globals
```


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
import { describe, it, expect } from 'vitest';
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
import { describe, it, expect } from 'vitest';
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
import { describe, it, expect } from 'vitest';
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
import { describe, it, expect } from 'vitest';
describe("sum", () => {
  it("adds", () => {
    expect(1 + 1).toBe(2);
  });
});
```

## Differences from the source

- Only a *call* of the bare name counts, as upstream: `vi.fn()` or `expect.extend(…)` is not reported, and `describe.each(…)(…)` is.
- A name declared anywhere in the file, including an import from another module, is not a global; upstream reports a name imported from elsewhere and then adds a clashing import.
- The fix on each finding carries every missing name, so one pass adds one import. It adds to an existing `vitest` import or `require('vitest')` destructuring, and starts a new import line when the existing one is `import type` or a namespace.
