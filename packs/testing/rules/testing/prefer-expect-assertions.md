---
id: testing/prefer-expect-assertions
title: "Async tests count their assertions"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: suggest
fix_reason: "`expect.hasAssertions()` fails a test that passed without asserting; the author may prefer `expect.assertions(n)`"
stability: preview
ported_from: ["eslint-plugin-jest/prefer-expect-assertions", "@vitest/eslint-plugin/prefer-expect-assertions"]
since: 0.5.0
---

An `async` test starts with `expect.hasAssertions()` or `expect.assertions(n)`. The suggested edit adds `expect.hasAssertions();`. Opt-in.

**Why:** An assertion in a callback or a `catch` may never run; counting assertions makes the test fail when it does not.

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
  kind: statement_block
  inside: { matches: test-fn, regex: '^async' }
  has: { nthChild: 1, pattern: $FIRST }
  not: { has: { stopBy: end, regex: '^expect\.(hasAssertions|assertions)\(' } }
message: "Add `expect.hasAssertions()` to this async test"
fix:
  - insert-before: { target: $FIRST, text: 'expect.hasAssertions(); ' }
```

## Examples

### JavaScript

#### Don't

```js
test("loads", async () => {
  await expect(load()).resolves.toBe(1);
});
```

#### Do

```js
test("loads", async () => {
  expect.hasAssertions(); await expect(load()).resolves.toBe(1);
});
```

### JSX

#### Don't

```jsx
test("loads", async () => {
  await expect(load()).resolves.toBe(1);
});
```

#### Do

```jsx
test("loads", async () => {
  expect.hasAssertions(); await expect(load()).resolves.toBe(1);
});
```

### TypeScript

#### Don't

```ts
test("loads", async () => {
  await expect(load()).resolves.toBe(1);
});
```

#### Do

```ts
test("loads", async () => {
  expect.hasAssertions(); await expect(load()).resolves.toBe(1);
});
```

### TSX

#### Don't

```tsx
test("loads", async () => {
  await expect(load()).resolves.toBe(1);
});
```

#### Do

```tsx
test("loads", async () => {
  expect.hasAssertions(); await expect(load()).resolves.toBe(1);
});
```
