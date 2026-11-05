---
id: testing/playwright-no-standalone-expect
title: "No `expect` outside a test or hook"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: manual
fix_reason: "the assertion moves into a test, which needs a title"
fix_exception: restructure
stability: preview
ported_from: ["eslint-plugin-playwright/no-standalone-expect"]
since: 0.5.0
---

`expect` is called inside a test or a hook, not at the top level or directly in a `describe` callback. The Playwright Test counterpart of `testing/no-standalone-expect`, for `test`, `test.describe` and `test.beforeEach`.

**Why:** An `expect` outside a test runs while the file is collected: its failure is reported as a broken file, or not at all.

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
  matches: expect-call
  not:
    inside:
      stopBy: end
      any:
        - matches: test-call
        - matches: hook-call
        - kind: function_declaration
        - kind: method_definition
        - { any: [{ kind: arrow_function }, { kind: function_expression }], not: { inside: { kind: arguments, inside: { matches: describe-call } } } }
message: "`expect` runs outside any test"
```

## How to fix

1. Move the assertion into a test (`test("…", () => { expect(…) })`) or a hook.

## Examples

### JavaScript

#### Don't

```js
describe("cart", () => {
  expect(cart.items).toHaveLength(0);
});
```

#### Do

```js
describe("cart", () => {
  test("starts empty", () => {
    expect(cart.items).toHaveLength(0);
  });
});
```

### JSX

#### Don't

```jsx
describe("cart", () => {
  expect(cart.items).toHaveLength(0);
});
```

#### Do

```jsx
describe("cart", () => {
  test("starts empty", () => {
    expect(cart.items).toHaveLength(0);
  });
});
```

### TypeScript

#### Don't

```ts
describe("cart", () => {
  expect(cart.items).toHaveLength(0);
});
```

#### Do

```ts
describe("cart", () => {
  test("starts empty", () => {
    expect(cart.items).toHaveLength(0);
  });
});
```

### TSX

#### Don't

```tsx
describe("cart", () => {
  expect(cart.items).toHaveLength(0);
});
```

#### Do

```tsx
describe("cart", () => {
  test("starts empty", () => {
    expect(cart.items).toHaveLength(0);
  });
});
```
