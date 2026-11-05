---
id: testing/playwright-valid-expect-in-promise
title: "A promise with assertions is awaited"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: manual
fix_reason: "the test becomes `async` and the chain is awaited or returned"
fix_exception: restructure
stability: preview
ported_from: ["eslint-plugin-playwright/valid-expect-in-promise"]
since: 0.5.0
---

In a test, a `.then`, `.catch` or `.finally` callback that calls `expect` belongs to a promise that is awaited or returned. The Playwright Test counterpart of `testing/valid-expect-in-promise`, for `test`, `test.describe` and `test.beforeEach`.

**Why:** Otherwise the test finishes before the callback runs, and its assertion is never counted.

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
  inside: { stopBy: end, matches: test-fn }
  has:
    kind: call_expression
    has: { field: function, regex: '\.(then|catch|finally)$' }
    all:
      - has: { field: arguments, has: { stopBy: end, matches: expect-call } }
message: "Await or return this promise, or its assertion may never run"
```

## How to fix

1. Make the test `async` and `await` the promise, or return it.

## Examples

### JavaScript

#### Don't

```js
test("loads", () => {
  load().then((v) => {
    expect(v).toBe(1);
  });
});
```

#### Do

```js
test("loads", async () => {
  await load().then((v) => {
    expect(v).toBe(1);
  });
});
```

### JSX

#### Don't

```jsx
test("loads", () => {
  load().then((v) => {
    expect(v).toBe(1);
  });
});
```

#### Do

```jsx
test("loads", async () => {
  await load().then((v) => {
    expect(v).toBe(1);
  });
});
```

### TypeScript

#### Don't

```ts
test("loads", () => {
  load().then((v) => {
    expect(v).toBe(1);
  });
});
```

#### Do

```ts
test("loads", async () => {
  await load().then((v) => {
    expect(v).toBe(1);
  });
});
```

### TSX

#### Don't

```tsx
test("loads", () => {
  load().then((v) => {
    expect(v).toBe(1);
  });
});
```

#### Do

```tsx
test("loads", async () => {
  await load().then((v) => {
    expect(v).toBe(1);
  });
});
```
