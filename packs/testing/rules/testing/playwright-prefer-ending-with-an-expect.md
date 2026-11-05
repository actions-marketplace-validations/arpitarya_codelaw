---
id: testing/playwright-prefer-ending-with-an-expect
title: "A test ends with an assertion"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: manual
fix_reason: "what to assert after the last action is the author's to write"
fix_exception: authored
stability: preview
ported_from: ["eslint-plugin-playwright/prefer-ending-with-an-expect"]
since: 0.5.0
---

A test's last statement is an `expect`. Opt-in. The Playwright Test counterpart of `testing/prefer-ending-with-an-expect`, for `test`, `test.describe` and `test.beforeEach`.

**Why:** Work after the last assertion is unchecked: it either belongs before the assertion or needs one of its own.

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
  inside: { matches: test-fn }
  has:
    nthChild: { position: 1, reverse: true }
    kind: expression_statement
    not: { has: { stopBy: end, matches: expect-call } }
message: "The test does not end with an assertion"
```

## How to fix

1. Add an assertion on the effect of the last statement, or move that statement before the last assertion.

## Examples

### JavaScript

#### Don't

```js
test("saves", () => {
  expect(form.valid).toBe(true);
  form.submit();
});
```

#### Do

```js
test("saves", () => {
  form.submit();
  expect(form.saved).toBe(true);
});
```

### JSX

#### Don't

```jsx
test("saves", () => {
  expect(form.valid).toBe(true);
  form.submit();
});
```

#### Do

```jsx
test("saves", () => {
  form.submit();
  expect(form.saved).toBe(true);
});
```

### TypeScript

#### Don't

```ts
test("saves", () => {
  expect(form.valid).toBe(true);
  form.submit();
});
```

#### Do

```ts
test("saves", () => {
  form.submit();
  expect(form.saved).toBe(true);
});
```

### TSX

#### Don't

```tsx
test("saves", () => {
  expect(form.valid).toBe(true);
  form.submit();
});
```

#### Do

```tsx
test("saves", () => {
  form.submit();
  expect(form.saved).toBe(true);
});
```
