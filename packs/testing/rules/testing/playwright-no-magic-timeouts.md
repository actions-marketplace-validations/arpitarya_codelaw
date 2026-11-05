---
id: testing/playwright-no-magic-timeouts
title: "No literal timeout values"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: manual
fix_reason: "the named constant, and where it lives, are the author's choice"
fix_exception: value
stability: preview
ported_from: ["eslint-plugin-playwright/no-magic-timeouts"]
since: 0.5.0
---

A `timeout` option, `test.setTimeout(…)` or `page.waitForTimeout(…)` is given a named constant, not a number literal. Opt-in.

**Why:** A bare number says nothing about why that wait is right, and the same value repeated drifts.

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
  kind: number
  any:
    - inside: { kind: pair, field: value, has: { field: key, regex: '^timeout$' } }
    - inside: { kind: arguments, inside: { kind: call_expression, has: { field: function, regex: '(^test\.setTimeout|\.waitForTimeout|\.setDefaultTimeout|\.setDefaultNavigationTimeout)$' } } }
message: "Name this timeout"
```

## How to fix

1. Declare the value once with a name (`const SLOW_API_MS = 10_000`) and use the name.

## Examples

### JavaScript

#### Don't

```js
test("t", async ({ page }) => {
  test.setTimeout(120000);
});
```

#### Do

```js
test("t", async ({ page }) => {
  test.setTimeout(SLOW_SUITE_MS);
});
```

### JSX

#### Don't

```jsx
test("t", async ({ page }) => {
  test.setTimeout(120000);
});
```

#### Do

```jsx
test("t", async ({ page }) => {
  test.setTimeout(SLOW_SUITE_MS);
});
```

### TypeScript

#### Don't

```ts
test("t", async ({ page }) => {
  test.setTimeout(120000);
});
```

#### Do

```ts
test("t", async ({ page }) => {
  test.setTimeout(SLOW_SUITE_MS);
});
```

### TSX

#### Don't

```tsx
test("t", async ({ page }) => {
  test.setTimeout(120000);
});
```

#### Do

```tsx
test("t", async ({ page }) => {
  test.setTimeout(SLOW_SUITE_MS);
});
```
