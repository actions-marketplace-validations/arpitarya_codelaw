---
id: testing/playwright-no-wait-for-navigation
title: "No `waitForNavigation`"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: suggest
fix_reason: "the replacement assertion on the URL needs the URL the author expects; deleting the call is the suggested half"
stability: preview
ported_from: ["eslint-plugin-playwright/no-wait-for-navigation"]
since: 0.5.0
---

A test does not call `page.waitForNavigation()`, which Playwright deprecates; it waits for the URL with `page.waitForURL(…)` or `expect(page).toHaveURL(…)`. The suggested edit deletes the call.

**Why:** `waitForNavigation` races the action that triggers it, so it can miss the navigation and hang.

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
  has: { any: [{ kind: call_expression, has: { field: function, regex: '\.waitForNavigation$' } }, { kind: await_expression, has: { kind: call_expression, has: { field: function, regex: '\.waitForNavigation$' } } }] }
message: "Wait for the URL instead of `waitForNavigation`"
fix: [remove]
```

## How to fix

1. After the action, assert the destination: `await expect(page).toHaveURL("/next")`.

## Examples

### JavaScript

#### Don't

```js
test("t", async ({ page }) => {
  await page.waitForNavigation();
  await expect(page).toHaveURL("/next");
});
```

#### Do

```js
test("t", async ({ page }) => {
  await expect(page).toHaveURL("/next");
});
```

### JSX

#### Don't

```jsx
test("t", async ({ page }) => {
  await page.waitForNavigation();
  await expect(page).toHaveURL("/next");
});
```

#### Do

```jsx
test("t", async ({ page }) => {
  await expect(page).toHaveURL("/next");
});
```

### TypeScript

#### Don't

```ts
test("t", async ({ page }) => {
  await page.waitForNavigation();
  await expect(page).toHaveURL("/next");
});
```

#### Do

```ts
test("t", async ({ page }) => {
  await expect(page).toHaveURL("/next");
});
```

### TSX

#### Don't

```tsx
test("t", async ({ page }) => {
  await page.waitForNavigation();
  await expect(page).toHaveURL("/next");
});
```

#### Do

```tsx
test("t", async ({ page }) => {
  await expect(page).toHaveURL("/next");
});
```
