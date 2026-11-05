---
id: testing/playwright-prefer-hooks-on-top
title: "Hooks before the tests"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: manual
fix_reason: "moving a hook above tests changes nothing at run time but the author places it"
fix_exception: restructure
stability: preview
ported_from: ["eslint-plugin-playwright/prefer-hooks-on-top"]
since: 0.5.0
---

In a block, every hook comes before the first test. The Playwright Test counterpart of `testing/prefer-hooks-on-top`, for `test`, `test.describe` and `test.beforeEach`.

**Why:** A hook applies to every test in its block, the ones above it too; declaring it after a test hides that.

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
  has: { matches: hook-call }
  follows: { stopBy: end, kind: expression_statement, has: { matches: test-call } }
message: "Declare hooks before the tests"
```

## How to fix

1. Move the hook above the first test of its block.

## Examples

### JavaScript

#### Don't

```js
test("a", () => {});
beforeEach(() => {
  setup();
});
```

#### Do

```js
beforeEach(() => {
  setup();
});
test("a", () => {});
```

### JSX

#### Don't

```jsx
test("a", () => {});
beforeEach(() => {
  setup();
});
```

#### Do

```jsx
beforeEach(() => {
  setup();
});
test("a", () => {});
```

### TypeScript

#### Don't

```ts
test("a", () => {});
beforeEach(() => {
  setup();
});
```

#### Do

```ts
beforeEach(() => {
  setup();
});
test("a", () => {});
```

### TSX

#### Don't

```tsx
test("a", () => {});
beforeEach(() => {
  setup();
});
```

#### Do

```tsx
beforeEach(() => {
  setup();
});
test("a", () => {});
```
