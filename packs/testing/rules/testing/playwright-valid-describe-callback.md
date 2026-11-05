---
id: testing/playwright-valid-describe-callback
title: "`describe` takes a plain synchronous callback"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: manual
fix_reason: "async work moves into a hook or test, and parameters out of the callback"
fix_exception: restructure
stability: preview
ported_from: ["eslint-plugin-playwright/valid-describe-callback"]
since: 0.5.0
---

A `describe` has a title and a callback; the callback is not `async`, takes no parameters (except in `describe.each`) and returns nothing. The Playwright Test counterpart of `testing/valid-describe-callback`, for `test`, `test.describe` and `test.beforeEach`.

**Why:** The runner calls the callback synchronously to collect tests: an async one registers its tests after collection is over, and they never run.

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
  kind: call_expression
  matches: describe-call
  not: { has: { field: function, regex: '\.each' } }
  any:
    - has: { field: arguments, not: { has: { nthChild: 2 } } }
    - has: { field: arguments, has: { nthChild: 2, any: [{ kind: arrow_function }, { kind: function_expression }], regex: '^async' } }
    - has: { field: arguments, has: { nthChild: 2, any: [{ kind: arrow_function }, { kind: function_expression }], has: { field: parameters, has: { nthChild: 1 } } } }
    - has: { field: arguments, has: { nthChild: 2, any: [{ kind: arrow_function }, { kind: function_expression }], has: { field: parameter, kind: identifier } } }
    - has: { field: arguments, has: { nthChild: 2, any: [{ kind: arrow_function }, { kind: function_expression }], has: { field: body, has: { kind: return_statement } } } }
message: "`describe` needs a synchronous callback with no parameters"
```

## How to fix

1. Move async setup into `beforeAll`.
2. Drop the callback's parameters and any `return`.

## Examples

### JavaScript

#### Don't

```js
describe("api", async () => {
  await connect();
  test("a", () => {});
});
```

#### Do

```js
describe("api", () => {
  beforeAll(async () => {
    await connect();
  });
  test("a", () => {});
});
```

### JSX

#### Don't

```jsx
describe("api", async () => {
  await connect();
  test("a", () => {});
});
```

#### Do

```jsx
describe("api", () => {
  beforeAll(async () => {
    await connect();
  });
  test("a", () => {});
});
```

### TypeScript

#### Don't

```ts
describe("api", async () => {
  await connect();
  test("a", () => {});
});
```

#### Do

```ts
describe("api", () => {
  beforeAll(async () => {
    await connect();
  });
  test("a", () => {});
});
```

### TSX

#### Don't

```tsx
describe("api", async () => {
  await connect();
  test("a", () => {});
});
```

#### Do

```tsx
describe("api", () => {
  beforeAll(async () => {
    await connect();
  });
  test("a", () => {});
});
```
