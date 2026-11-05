---
id: testing/playwright-no-raw-locators
title: "No CSS or XPath locators"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: manual
fix_reason: "the locator becomes one by role, label or text, which needs knowing what the user sees"
fix_exception: restructure
stability: preview
ported_from: ["eslint-plugin-playwright/no-raw-locators"]
since: 0.5.0
---

A test locates elements with Playwright's user-facing locators (`getByRole`, `getByLabel`, `getByText`, …), not `page.locator("<css or xpath>")`. Opt-in.

**Why:** CSS and XPath tie the test to markup; user-facing locators survive refactors and check accessibility on the way.

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
  all:
    - has: { field: function, kind: member_expression, has: { field: property, regex: '^locator$' } }
    - has: { field: arguments, has: { nthChild: 1, kind: string } }
message: "Use a user-facing locator"
```

## How to fix

1. Replace it with `getByRole`, `getByLabel`, `getByText` or `getByTestId`.

## Examples

### JavaScript

#### Don't

```js
test("t", async ({ page }) => {
  await page.locator(".save").click();
});
```

#### Do

```js
test("t", async ({ page }) => {
  await page.getByRole("button", { name: "Save" }).click();
});
```

### JSX

#### Don't

```jsx
test("t", async ({ page }) => {
  await page.locator(".save").click();
});
```

#### Do

```jsx
test("t", async ({ page }) => {
  await page.getByRole("button", { name: "Save" }).click();
});
```

### TypeScript

#### Don't

```ts
test("t", async ({ page }) => {
  await page.locator(".save").click();
});
```

#### Do

```ts
test("t", async ({ page }) => {
  await page.getByRole("button", { name: "Save" }).click();
});
```

### TSX

#### Don't

```tsx
test("t", async ({ page }) => {
  await page.locator(".save").click();
});
```

#### Do

```tsx
test("t", async ({ page }) => {
  await page.getByRole("button", { name: "Save" }).click();
});
```
