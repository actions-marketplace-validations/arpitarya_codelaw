---
id: testing/prefer-describe-function-title
title: "`describe(fn)` for a block about an imported function"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: auto
stability: preview
ported_from: ["@vitest/eslint-plugin/prefer-describe-function-title"]
since: 0.5.0
---

A `describe` whose title is the name of a function the file imports passes the function itself: `describe(add, …)`, not `describe("add", …)`.

**Why:** Vitest takes the title from the function's name, so a rename of the function carries to the test, and the editor can jump to it.

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
  kind: string
  nthChild: 1
  has: { kind: string_fragment, pattern: $N }
  all:
    - inside: { kind: arguments, inside: { matches: describe-call } }
    - inside:
        stopBy: end
        kind: program
        has: { kind: import_statement, has: { stopBy: end, kind: import_specifier, not: { has: { field: alias, kind: identifier } }, has: { field: name, pattern: $N } } }
message: "Pass the function, not its name"
fix: '$N'
```

## Examples

### JavaScript

#### Don't

```js
import { add } from "./math";
describe("add", () => {
  test("adds", () => {});
});
```

#### Do

```js
import { add } from "./math";
describe(add, () => {
  test("adds", () => {});
});
```

### JSX

#### Don't

```jsx
import { add } from "./math";
describe("add", () => {
  test("adds", () => {});
});
```

#### Do

```jsx
import { add } from "./math";
describe(add, () => {
  test("adds", () => {});
});
```

### TypeScript

#### Don't

```ts
import { add } from "./math";
describe("add", () => {
  test("adds", () => {});
});
```

#### Do

```ts
import { add } from "./math";
describe(add, () => {
  test("adds", () => {});
});
```

### TSX

#### Don't

```tsx
import { add } from "./math";
describe("add", () => {
  test("adds", () => {});
});
```

#### Do

```tsx
import { add } from "./math";
describe(add, () => {
  test("adds", () => {});
});
```
