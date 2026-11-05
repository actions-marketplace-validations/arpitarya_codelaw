---
id: testing/playwright-no-wait-for-timeout
title: "No fixed sleeps"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: suggest
fix_reason: "without the sleep the next line runs at once; it may need a web-first assertion in its place"
stability: preview
ported_from: ["eslint-plugin-playwright/no-wait-for-timeout"]
since: 0.5.0
---

A test does not call `page.waitForTimeout(ms)`. The suggested edit deletes the call; wait for the state the test needs instead.

**Why:** A fixed sleep is too short on a slow machine and wasted time on a fast one: the classic flaky test.

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
  has: { any: [{ kind: call_expression, has: { field: function, regex: '\.waitForTimeout$' } }, { kind: await_expression, has: { kind: call_expression, has: { field: function, regex: '\.waitForTimeout$' } } }] }
message: "Wait for a state, not a time"
fix: [remove]
```

## Examples

### JavaScript

#### Don't

```js
test("t", async ({ page }) => {
  await page.waitForTimeout(1000);
  await expect(page.getByText("Done")).toBeVisible();
});
```

#### Do

```js
test("t", async ({ page }) => {
  await expect(page.getByText("Done")).toBeVisible();
});
```

### JSX

#### Don't

```jsx
test("t", async ({ page }) => {
  await page.waitForTimeout(1000);
  await expect(page.getByText("Done")).toBeVisible();
});
```

#### Do

```jsx
test("t", async ({ page }) => {
  await expect(page.getByText("Done")).toBeVisible();
});
```

### TypeScript

#### Don't

```ts
test("t", async ({ page }) => {
  await page.waitForTimeout(1000);
  await expect(page.getByText("Done")).toBeVisible();
});
```

#### Do

```ts
test("t", async ({ page }) => {
  await expect(page.getByText("Done")).toBeVisible();
});
```

### TSX

#### Don't

```tsx
test("t", async ({ page }) => {
  await page.waitForTimeout(1000);
  await expect(page.getByText("Done")).toBeVisible();
});
```

#### Do

```tsx
test("t", async ({ page }) => {
  await expect(page.getByText("Done")).toBeVisible();
});
```
