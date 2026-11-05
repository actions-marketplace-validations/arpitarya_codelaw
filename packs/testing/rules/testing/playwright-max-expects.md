---
id: testing/playwright-max-expects
title: "At most five assertions in a test"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: manual
fix_reason: "splitting the test decides which assertions belong together"
fix_exception: restructure
stability: preview
ported_from: ["eslint-plugin-playwright/max-expects"]
since: 0.5.0
---

A test body holds at most five `expect` statements; the sixth is reported. (The source's default; the limit is fixed here, and only statements directly in the test body are counted.) The Playwright Test counterpart of `testing/max-expects`, for `test`, `test.describe` and `test.beforeEach`.

**Why:** A test that asserts many things stops at the first failure and hides the rest; smaller tests each report their own.

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
  all:
    - inside: { kind: statement_block, inside: { matches: test-fn } }
    - has: { stopBy: end, matches: expect-call }
    - nthChild: { position: 6, ofRule: { kind: expression_statement, has: { stopBy: end, matches: expect-call } } }
message: "This test has more than five assertions; split it"
```

## How to fix

1. Split the test into several, each asserting one behaviour.

## Examples

### JavaScript

#### Don't

```js
test("user", () => {
  expect(u.a).toBe(1);
  expect(u.b).toBe(2);
  expect(u.c).toBe(3);
  expect(u.d).toBe(4);
  expect(u.e).toBe(5);
  expect(u.f).toBe(6);
});
```

#### Do

```js
test("user", () => {
  expect(u).toMatchObject({ a: 1, b: 2, c: 3, d: 4, e: 5, f: 6 });
});
```

### JSX

#### Don't

```jsx
test("user", () => {
  expect(u.a).toBe(1);
  expect(u.b).toBe(2);
  expect(u.c).toBe(3);
  expect(u.d).toBe(4);
  expect(u.e).toBe(5);
  expect(u.f).toBe(6);
});
```

#### Do

```jsx
test("user", () => {
  expect(u).toMatchObject({ a: 1, b: 2, c: 3, d: 4, e: 5, f: 6 });
});
```

### TypeScript

#### Don't

```ts
test("user", () => {
  expect(u.a).toBe(1);
  expect(u.b).toBe(2);
  expect(u.c).toBe(3);
  expect(u.d).toBe(4);
  expect(u.e).toBe(5);
  expect(u.f).toBe(6);
});
```

#### Do

```ts
test("user", () => {
  expect(u).toMatchObject({ a: 1, b: 2, c: 3, d: 4, e: 5, f: 6 });
});
```

### TSX

#### Don't

```tsx
test("user", () => {
  expect(u.a).toBe(1);
  expect(u.b).toBe(2);
  expect(u.c).toBe(3);
  expect(u.d).toBe(4);
  expect(u.e).toBe(5);
  expect(u.f).toBe(6);
});
```

#### Do

```tsx
test("user", () => {
  expect(u).toMatchObject({ a: 1, b: 2, c: 3, d: 4, e: 5, f: 6 });
});
```
