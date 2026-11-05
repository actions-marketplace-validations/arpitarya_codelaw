---
id: testing/playwright-no-unused-locators
title: "No locator created and left unused"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: manual
fix_reason: "what the locator was for — an action or an assertion — is the author's to write"
fix_exception: authored
stability: preview
ported_from: ["eslint-plugin-playwright/no-unused-locators"]
since: 0.5.0
---

A locator (`page.getByRole(…)`, `page.locator(…)`) is not created as a statement on its own: it does nothing until it is acted on or asserted.

**Why:** Locators are lazy, so a bare one neither waits nor checks; it reads as if it asserted that the element exists.

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
    - inside: { stopBy: end, matches: test-fn }
    - has:
        nthChild: 1
        any:
          - { kind: call_expression, has: { field: function, kind: member_expression, has: { field: property, regex: '^(locator|getByRole|getByText|getByLabel|getByPlaceholder|getByAltText|getByTitle|getByTestId|frameLocator|first|last|nth|filter|and|or)$' } } }
          - { kind: await_expression, has: { kind: call_expression, has: { field: function, kind: member_expression, has: { field: property, regex: '^(locator|getByRole|getByText|getByLabel|getByPlaceholder|getByAltText|getByTitle|getByTestId|frameLocator|first|last|nth|filter|and|or)$' } } } }
message: "This locator is never used"
```

## How to fix

1. Act on it (`.click()`), assert on it (`await expect(…).toBeVisible()`), or delete it.

## Examples

### JavaScript

#### Don't

```js
test("t", async ({ page }) => {
  page.getByRole("button", { name: "Save" });
});
```

#### Do

```js
test("t", async ({ page }) => {
  await expect(page.getByRole("button", { name: "Save" })).toBeVisible();
});
```

### JSX

#### Don't

```jsx
test("t", async ({ page }) => {
  page.getByRole("button", { name: "Save" });
});
```

#### Do

```jsx
test("t", async ({ page }) => {
  await expect(page.getByRole("button", { name: "Save" })).toBeVisible();
});
```

### TypeScript

#### Don't

```ts
test("t", async ({ page }) => {
  page.getByRole("button", { name: "Save" });
});
```

#### Do

```ts
test("t", async ({ page }) => {
  await expect(page.getByRole("button", { name: "Save" })).toBeVisible();
});
```

### TSX

#### Don't

```tsx
test("t", async ({ page }) => {
  page.getByRole("button", { name: "Save" });
});
```

#### Do

```tsx
test("t", async ({ page }) => {
  await expect(page.getByRole("button", { name: "Save" })).toBeVisible();
});
```
