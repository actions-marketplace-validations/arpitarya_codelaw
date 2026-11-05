---
id: testing/playwright-no-conditional-in-test
title: "No conditional logic in a test"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: manual
fix_reason: "each branch becomes its own test, with its own setup; how to split is the author's call"
fix_exception: restructure
stability: preview
ported_from: ["eslint-plugin-playwright/no-conditional-in-test"]
since: 0.5.0
---

A test body has no `if`, `switch` or `? :`: each test runs one path. The Playwright Test counterpart of `testing/no-conditional-in-test`, for `test`, `test.describe` and `test.beforeEach`.

**Why:** A test that branches checks different things on different runs; a failure is harder to trace and a path can go untested.

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
  any: [{ kind: if_statement }, { kind: switch_statement }, { kind: ternary_expression }]
  inside: { stopBy: end, matches: test-fn }
message: "A test should not branch; split it"
```

## How to fix

1. Write one test per branch, each with the setup that makes that branch happen.
2. Use `test.each` when the branches differ only in data.

## Examples

### JavaScript

#### Don't

```js
test("formats", () => {
  if (isWindows) {
    expect(fmt()).toBe("a\\b");
  }
});
```

#### Do

```js
test("formats", () => {
  expect(fmt()).toBe("a/b");
});
```

### JSX

#### Don't

```jsx
test("formats", () => {
  if (isWindows) {
    expect(fmt()).toBe("a\\b");
  }
});
```

#### Do

```jsx
test("formats", () => {
  expect(fmt()).toBe("a/b");
});
```

### TypeScript

#### Don't

```ts
test("formats", () => {
  if (isWindows) {
    expect(fmt()).toBe("a\\b");
  }
});
```

#### Do

```ts
test("formats", () => {
  expect(fmt()).toBe("a/b");
});
```

### TSX

#### Don't

```tsx
test("formats", () => {
  if (isWindows) {
    expect(fmt()).toBe("a\\b");
  }
});
```

#### Do

```tsx
test("formats", () => {
  expect(fmt()).toBe("a/b");
});
```
