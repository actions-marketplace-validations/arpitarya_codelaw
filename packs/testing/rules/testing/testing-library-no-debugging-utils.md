---
id: testing/testing-library-no-debugging-utils
title: "No `debug()` left in a test"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: suggest
fix_reason: "the call only prints; removing it is safe but the author may still be using it"
stability: preview
ported_from: ["eslint-plugin-testing-library/no-debugging-utils"]
since: 0.5.0
---

`screen.debug()`, a render result's `debug()`, `prettyDOM`, `logRoles`, `logDOM` and `logTestingPlaygroundURL` are not left in a test. The suggested edit deletes the statement.

**Why:** Debug output floods the test log and slows the run; it is meant for a one-off look.

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
  kind: expression_statement
  all:
    - has: { kind: call_expression, has: { field: function, regex: '^(screen\.debug|debug|screen\.logTestingPlaygroundURL|logRoles|logDOM|prettyDOM|\w+\.debug)$' } }
    - inside: { stopBy: end, kind: program, has: { kind: import_statement, has: { field: source, regex: '@testing-library/' } } }
message: "Remove this debugging call"
fix: [remove]
```

## Examples

### JavaScript

#### Don't

```js
import { screen } from "@testing-library/react";
test("t", () => {
  screen.debug();
  expect(screen.getByText("Hi")).toBeVisible();
});
```

#### Do

```js
import { screen } from "@testing-library/react";
test("t", () => {
  expect(screen.getByText("Hi")).toBeVisible();
});
```

### JSX

#### Don't

```jsx
import { screen } from "@testing-library/react";
test("t", () => {
  screen.debug();
  expect(screen.getByText("Hi")).toBeVisible();
});
```

#### Do

```jsx
import { screen } from "@testing-library/react";
test("t", () => {
  expect(screen.getByText("Hi")).toBeVisible();
});
```

### TypeScript

#### Don't

```ts
import { screen } from "@testing-library/react";
test("t", () => {
  screen.debug();
  expect(screen.getByText("Hi")).toBeVisible();
});
```

#### Do

```ts
import { screen } from "@testing-library/react";
test("t", () => {
  expect(screen.getByText("Hi")).toBeVisible();
});
```

### TSX

#### Don't

```tsx
import { screen } from "@testing-library/react";
test("t", () => {
  screen.debug();
  expect(screen.getByText("Hi")).toBeVisible();
});
```

#### Do

```tsx
import { screen } from "@testing-library/react";
test("t", () => {
  expect(screen.getByText("Hi")).toBeVisible();
});
```
