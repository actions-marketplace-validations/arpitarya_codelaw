---
id: testing/expect-expect
title: "Every test asserts"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: manual
fix_reason: "the assertion a test is missing is the author's to write"
fix_exception: authored
stability: preview
ported_from: ["eslint-plugin-jest/expect-expect", "@vitest/eslint-plugin/expect-expect"]
since: 0.5.0
---

A test body calls `expect` (or `expect.soft`, `expect.poll`), Vitest's `expectTypeOf` or `assertType`, or an `assert` at least once. `test.todo` has no body and is left alone.

**Why:** A test with no assertion passes as long as nothing throws, so it reads as coverage and checks nothing.

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
  matches: test-fn
  not: { has: { stopBy: end, kind: identifier, regex: '^(expect|expectTypeOf|assert|assertType)$' } }
message: "This test has no assertion"
```

## How to fix

1. Add an `expect` on the result the test is about.
2. If the test only checks that nothing throws, say so: `expect(() => run()).not.toThrow()`.

## Examples

### JavaScript

#### Don't

```js
test("saves", () => {
  save(user);
});
```

#### Do

```js
test("saves", () => {
  save(user);
  expect(store.get(1)).toStrictEqual(user);
});
```

### JSX

#### Don't

```jsx
test("saves", () => {
  save(user);
});
```

#### Do

```jsx
test("saves", () => {
  save(user);
  expect(store.get(1)).toStrictEqual(user);
});
```

### TypeScript

#### Don't

```ts
test("saves", () => {
  save(user);
});
```

#### Do

```ts
test("saves", () => {
  save(user);
  expect(store.get(1)).toStrictEqual(user);
});
```

### TSX

#### Don't

```tsx
test("saves", () => {
  save(user);
});
```

#### Do

```tsx
test("saves", () => {
  save(user);
  expect(store.get(1)).toStrictEqual(user);
});
```
