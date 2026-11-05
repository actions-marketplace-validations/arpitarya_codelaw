---
id: testing/playwright-no-test-return-statement
title: "No `return` from a test"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: manual
fix_reason: "`await` needs the callback to be `async`, and a returned value that is not a promise is a mistake to remove"
fix_exception: restructure
stability: preview
ported_from: ["eslint-plugin-playwright/no-test-return-statement"]
since: 0.5.0
---

A test callback does not `return` a value; an async test is `async` and `await`s. The Playwright Test counterpart of `testing/no-test-return-statement`, for `test`, `test.describe` and `test.beforeEach`.

**Why:** A returned promise is easy to lose in a refactor, and then the test passes before the assertion runs; `await` reads plainly.

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
  kind: return_statement
  has: { nthChild: 1 }
  inside: { kind: statement_block, inside: { matches: test-fn } }
message: "Await in an `async` test instead of returning"
```

## How to fix

1. Make the callback `async`.
2. Replace `return <promise>` with `await <promise>`.

## Examples

### JavaScript

#### Don't

```js
test("loads", () => {
  return expect(load()).resolves.toBe(1);
});
```

#### Do

```js
test("loads", async () => {
  await expect(load()).resolves.toBe(1);
});
```

### JSX

#### Don't

```jsx
test("loads", () => {
  return expect(load()).resolves.toBe(1);
});
```

#### Do

```jsx
test("loads", async () => {
  await expect(load()).resolves.toBe(1);
});
```

### TypeScript

#### Don't

```ts
test("loads", () => {
  return expect(load()).resolves.toBe(1);
});
```

#### Do

```ts
test("loads", async () => {
  await expect(load()).resolves.toBe(1);
});
```

### TSX

#### Don't

```tsx
test("loads", () => {
  return expect(load()).resolves.toBe(1);
});
```

#### Do

```tsx
test("loads", async () => {
  await expect(load()).resolves.toBe(1);
});
```
