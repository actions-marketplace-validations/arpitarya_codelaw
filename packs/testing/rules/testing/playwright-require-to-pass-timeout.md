---
id: testing/playwright-require-to-pass-timeout
title: "`toPass` with a timeout"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: manual
fix_reason: "how long the block may retry depends on what it waits for"
fix_exception: value
stability: preview
ported_from: ["eslint-plugin-playwright/require-to-pass-timeout"]
since: 0.5.0
---

`expect(…).toPass()` is given `{ timeout }`. Opt-in.

**Why:** Without one `toPass` retries until the test's own timeout, so a stuck check uses up the whole test.

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
  kind: call_expression
  matches: matcher-call
  all:
    - has: { field: function, has: { field: property, regex: '^toPass$' } }
    - not: { has: { field: arguments, has: { kind: object, has: { kind: pair, has: { field: key, regex: '^timeout$' } } } } }
message: "Give `toPass` a timeout"
```

## How to fix

1. Pass a timeout: `toPass({ timeout: 10_000 })`.

## Examples

### JavaScript

#### Don't

```js
test("t", async () => {
  await expect(async () => check()).toPass();
});
```

#### Do

```js
test("t", async () => {
  await expect(async () => check()).toPass({ timeout: 10000 });
});
```

### JSX

#### Don't

```jsx
test("t", async () => {
  await expect(async () => check()).toPass();
});
```

#### Do

```jsx
test("t", async () => {
  await expect(async () => check()).toPass({ timeout: 10000 });
});
```

### TypeScript

#### Don't

```ts
test("t", async () => {
  await expect(async () => check()).toPass();
});
```

#### Do

```ts
test("t", async () => {
  await expect(async () => check()).toPass({ timeout: 10000 });
});
```

### TSX

#### Don't

```tsx
test("t", async () => {
  await expect(async () => check()).toPass();
});
```

#### Do

```tsx
test("t", async () => {
  await expect(async () => check()).toPass({ timeout: 10000 });
});
```
