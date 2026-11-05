---
id: testing/no-done-callback
title: "No `done` callback in tests and hooks"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: manual
fix_reason: "the callback-style code becomes `async`/`await` or a returned promise, which changes the body's shape"
fix_exception: restructure
stability: preview
ported_from: ["eslint-plugin-jest/no-done-callback", "@vitest/eslint-plugin/no-done-callback"]
since: 0.5.0
---

A test or hook callback takes no `done` parameter; asynchronous work is `async`/`await`. `test.each` callbacks, whose parameters are the table's values, are left alone.

**Why:** A `done` that is never called times the test out, and an exception thrown before it is called is reported as a timeout, not as the error.

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
  any: [{ kind: arrow_function }, { kind: function_expression }]
  all:
    - not: { regex: '^async' }
    - any:
      - has: { field: parameters, has: { nthChild: 1 } }
      - has: { field: parameter, kind: identifier }
  inside:
    kind: arguments
    inside:
      any:
        - { matches: test-call, not: { has: { field: function, regex: '\.each' } } }
        - matches: hook-call
message: "Use `async`/`await` instead of a `done` callback"
```

## How to fix

1. Make the callback `async` and drop the parameter.
2. `await` the work; wrap a callback API in a `Promise` if it has no promise form.

## Examples

### JavaScript

#### Don't

```js
test("loads", (done) => {
  load(() => {
    done();
  });
});
```

#### Do

```js
test("loads", async () => {
  await new Promise((resolve) => load(resolve));
});
```

### JSX

#### Don't

```jsx
test("loads", (done) => {
  load(() => {
    done();
  });
});
```

#### Do

```jsx
test("loads", async () => {
  await new Promise((resolve) => load(resolve));
});
```

### TypeScript

#### Don't

```ts
test("loads", (done) => {
  load(() => {
    done();
  });
});
```

#### Do

```ts
test("loads", async () => {
  await new Promise((resolve) => load(resolve));
});
```

### TSX

#### Don't

```tsx
test("loads", (done) => {
  load(() => {
    done();
  });
});
```

#### Do

```tsx
test("loads", async () => {
  await new Promise((resolve) => load(resolve));
});
```
