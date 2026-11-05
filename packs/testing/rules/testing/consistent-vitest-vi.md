---
id: testing/consistent-vitest-vi
title: "`vi`, not `vitest`, for Vitest's utilities"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: auto
stability: preview
ported_from: ["@vitest/eslint-plugin/consistent-vitest-vi"]
since: 0.5.0
---

Vitest's utility object is used as `vi` (`vi.fn()`, `vi.mock()`), not by its alias `vitest`; the import is renamed with its uses.

**Why:** Both names are the same object; one name keeps the suite searchable.

```codelaw
utils:
  expect-call:
    kind: call_expression
    has:
      field: function
      any:
        - { kind: identifier, regex: '^expect$' }
        - kind: member_expression
          all:
            - has: { field: property, regex: '^(soft|poll)$' }
            - has: { field: object, kind: identifier, regex: '^expect$' }
  expect-chain:
    any:
      - matches: expect-call
      - kind: member_expression
        has:
          field: object
          any:
            - matches: expect-call
            - { kind: member_expression, has: { field: object, matches: expect-call } }
  matcher-call:
    kind: call_expression
    has:
      field: function
      kind: member_expression
      has: { field: object, matches: expect-chain }
  test-name:
    any:
      - { kind: identifier, regex: '^(it|test|xit|xtest|fit)$' }
      - kind: member_expression
        all:
          - has: { field: property, regex: '^(only|skip|concurrent|failing|todo|fixme|fail|slow)$' }
          - regex: '^(it|test|xit|xtest|fit)(\.(only|skip|concurrent|failing|todo|fixme|fail|slow))+$'
  describe-name:
    any:
      - { kind: identifier, regex: '^(describe|xdescribe|fdescribe|suite)$' }
      - kind: member_expression
        all:
          - has: { field: property, regex: '^(describe|only|skip|serial|parallel|concurrent|fixme)$' }
          - regex: '^(describe|xdescribe|fdescribe|test\.describe|suite)(\.(only|skip|serial|parallel|concurrent|fixme))*$'
  test-call:
    kind: call_expression
    has:
      field: function
      any:
        - matches: test-name
        - kind: call_expression
          has:
            field: function
            kind: member_expression
            all:
              - has: { field: property, regex: '^each$' }
              - regex: '^(it|test|xit|xtest|fit)(\.(only|skip|concurrent|failing))*\.each$'
  describe-call:
    kind: call_expression
    has:
      field: function
      any:
        - matches: describe-name
        - kind: call_expression
          has:
            field: function
            kind: member_expression
            all:
              - has: { field: property, regex: '^each$' }
              - regex: '^(describe|xdescribe|fdescribe|suite)(\.(only|skip|concurrent))*\.each$'
  hook-call:
    kind: call_expression
    has:
      field: function
      any:
        - { kind: identifier, regex: '^(beforeAll|beforeEach|afterAll|afterEach)$' }
        - kind: member_expression
          all:
            - has: { field: property, regex: '^(beforeAll|beforeEach|afterAll|afterEach)$' }
            - has: { field: object, kind: identifier, regex: '^test$' }
  test-fn:
    any: [{ kind: arrow_function }, { kind: function_expression }]
    inside: { kind: arguments, inside: { matches: test-call } }
  describe-fn:
    any: [{ kind: arrow_function }, { kind: function_expression }]
    inside: { kind: arguments, inside: { matches: describe-call } }
  in-test:
    inside: { stopBy: end, any: [{ matches: test-call }, { matches: hook-call }] }
  block-level:
    kind: expression_statement
    any:
      - inside: { kind: program }
      - inside: { kind: statement_block, inside: { matches: describe-fn } }
match:
  any:
    - kind: identifier
      regex: '^vitest$'
      pattern: $U
      inside: { kind: member_expression, field: object }
    - kind: identifier
      regex: '^vitest$'
      pattern: $I
      inside: { kind: import_specifier, inside: { stopBy: end, kind: import_statement, has: { field: source, regex: '^["'']vitest["'']$' } } }
message: "Use `vi` instead of `vitest`"
fix:
  - replace: { target: $U, with: vi }
  - replace: { target: $I, with: vi }
```

## Examples

### JavaScript

#### Don't

```js
import { test, vitest } from "vitest";
test("t", () => {
  const f = vitest.fn();
});
```

#### Do

```js
import { test, vi } from "vitest";
test("t", () => {
  const f = vi.fn();
});
```

### JSX

#### Don't

```jsx
import { test, vitest } from "vitest";
test("t", () => {
  const f = vitest.fn();
});
```

#### Do

```jsx
import { test, vi } from "vitest";
test("t", () => {
  const f = vi.fn();
});
```

### TypeScript

#### Don't

```ts
import { test, vitest } from "vitest";
test("t", () => {
  const f = vitest.fn();
});
```

#### Do

```ts
import { test, vi } from "vitest";
test("t", () => {
  const f = vi.fn();
});
```

### TSX

#### Don't

```tsx
import { test, vitest } from "vitest";
test("t", () => {
  const f = vitest.fn();
});
```

#### Do

```tsx
import { test, vi } from "vitest";
test("t", () => {
  const f = vi.fn();
});
```
