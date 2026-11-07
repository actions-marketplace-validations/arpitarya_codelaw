---
id: testing/playwright-no-networkidle
title: "No `networkidle`"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: manual
fix_reason: "the wait becomes an assertion on what the page shows, which needs knowing what it waits for"
fix_exception: restructure
stability: preview
ported_from: ["eslint-plugin-playwright/no-networkidle", "eslint-plugin-sonarjs/no-networkidle-wait"]
since: 0.5.0
---

A test does not wait for the `"networkidle"` load state (in `waitForLoadState`, `goto`, `waitForURL` or `reload`).

**Why:** Network-idle is a guess: pages that poll never reach it and pages that load lazily reach it early. A web-first assertion waits for what the test needs.

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
  kind: string
  regex: '^["'']networkidle["'']$'
message: "Wait for what the test needs instead of `networkidle`"
```

## How to fix

1. Replace the wait with a web-first assertion on the element the test needs: `await expect(page.getByRole("table")).toBeVisible()`.

## Examples

### JavaScript

#### Don't

```js
test("t", async ({ page }) => {
  await page.waitForLoadState("networkidle");
});
```

#### Do

```js
test("t", async ({ page }) => {
  await expect(page.getByRole("table")).toBeVisible();
});
```

### JSX

#### Don't

```jsx
test("t", async ({ page }) => {
  await page.waitForLoadState("networkidle");
});
```

#### Do

```jsx
test("t", async ({ page }) => {
  await expect(page.getByRole("table")).toBeVisible();
});
```

### TypeScript

#### Don't

```ts
test("t", async ({ page }) => {
  await page.waitForLoadState("networkidle");
});
```

#### Do

```ts
test("t", async ({ page }) => {
  await expect(page.getByRole("table")).toBeVisible();
});
```

### TSX

#### Don't

```tsx
test("t", async ({ page }) => {
  await page.waitForLoadState("networkidle");
});
```

#### Do

```tsx
test("t", async ({ page }) => {
  await expect(page.getByRole("table")).toBeVisible();
});
```
