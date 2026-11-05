---
id: testing/playwright-require-hook
title: "Setup code in a hook"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: manual
fix_reason: "the code moves into a `beforeAll` or `beforeEach`, chosen by when it must run"
fix_exception: restructure
stability: preview
ported_from: ["eslint-plugin-playwright/require-hook"]
since: 0.5.0
---

A call at the top level of a test file or directly in a `describe` callback — other than a test, a `describe`, a hook or a `jest.*`/`vi.*` call — is in a hook. The Playwright Test counterpart of `testing/require-hook`, for `test`, `test.describe` and `test.beforeEach`.

**Why:** Code outside a hook runs while tests are collected, before any of them, even when the test that needs it is filtered out.

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
  matches: block-level
  has:
    kind: call_expression
    not:
      any:
        - matches: test-call
        - matches: describe-call
        - matches: hook-call
        - has: { field: function, regex: '^(jest|vi|expect|test|require)\b' }
message: "Run setup code in a hook"
```

## How to fix

1. Wrap the statement in `beforeAll` (once) or `beforeEach` (per test).

## Examples

### JavaScript

#### Don't

```js
describe("db", () => {
  seed();
  test("reads", () => {});
});
```

#### Do

```js
describe("db", () => {
  beforeAll(() => {
    seed();
  });
  test("reads", () => {});
});
```

### JSX

#### Don't

```jsx
describe("db", () => {
  seed();
  test("reads", () => {});
});
```

#### Do

```jsx
describe("db", () => {
  beforeAll(() => {
    seed();
  });
  test("reads", () => {});
});
```

### TypeScript

#### Don't

```ts
describe("db", () => {
  seed();
  test("reads", () => {});
});
```

#### Do

```ts
describe("db", () => {
  beforeAll(() => {
    seed();
  });
  test("reads", () => {});
});
```

### TSX

#### Don't

```tsx
describe("db", () => {
  seed();
  test("reads", () => {});
});
```

#### Do

```tsx
describe("db", () => {
  beforeAll(() => {
    seed();
  });
  test("reads", () => {});
});
```
