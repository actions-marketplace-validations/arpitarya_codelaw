---
id: testing/playwright-missing-playwright-await
title: "`await` Playwright's async assertions and steps"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: auto
stability: preview
ported_from: ["eslint-plugin-playwright/missing-playwright-await"]
since: 0.5.0
---

A web-first assertion (`toBeVisible`, `toHaveText`, `toHaveURL`, …), a `.resolves`/`.rejects` assertion, `expect.poll(…)` and `test.step(…)` are awaited. The fix adds `await` inside an `async` function.

**Why:** Each returns a promise; without `await` the test ends before the retrying assertion has finished, and its failure is lost.

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
  pattern: $AW
  all:
    - inside: { kind: expression_statement }
    - inside: { stopBy: end, any: [{ kind: arrow_function }, { kind: function_expression }, { kind: function_declaration }], regex: '^async' }
    - any:
      - { matches: matcher-call, has: { field: function, has: { field: property, regex: '^(toBeAttached|toBeChecked|toBeDisabled|toBeEditable|toBeEmpty|toBeEnabled|toBeFocused|toBeHidden|toBeInViewport|toBeVisible|toContainText|toContainClass|toHaveAccessibleDescription|toHaveAccessibleErrorMessage|toHaveAccessibleName|toHaveAttribute|toHaveClass|toHaveCount|toHaveCSS|toHaveId|toHaveJSProperty|toHaveRole|toHaveScreenshot|toHaveText|toHaveTitle|toHaveURL|toHaveValue|toHaveValues|toPass|toBeOK|toMatchAriaSnapshot)$' } } }
      - { matches: matcher-call, has: { field: function, regex: '\.(resolves|rejects)\.' } }
      - { has: { field: function, kind: member_expression, has: { field: object, stopBy: end, kind: call_expression, has: { field: function, regex: '^expect\.poll$' } } } }
      - { has: { field: function, regex: '^test\.step$' } }
message: "Await this Playwright call"
fix:
  - insert-before: { target: $AW, text: 'await ' }
```

## Examples

### JavaScript

#### Don't

```js
test("shows", async ({ page }) => {
  expect(page.getByText("Hi")).toBeVisible();
});
```

#### Do

```js
test("shows", async ({ page }) => {
  await expect(page.getByText("Hi")).toBeVisible();
});
```

### JSX

#### Don't

```jsx
test("shows", async ({ page }) => {
  expect(page.getByText("Hi")).toBeVisible();
});
```

#### Do

```jsx
test("shows", async ({ page }) => {
  await expect(page.getByText("Hi")).toBeVisible();
});
```

### TypeScript

#### Don't

```ts
test("shows", async ({ page }) => {
  expect(page.getByText("Hi")).toBeVisible();
});
```

#### Do

```ts
test("shows", async ({ page }) => {
  await expect(page.getByText("Hi")).toBeVisible();
});
```

### TSX

#### Don't

```tsx
test("shows", async ({ page }) => {
  expect(page.getByText("Hi")).toBeVisible();
});
```

#### Do

```tsx
test("shows", async ({ page }) => {
  await expect(page.getByText("Hi")).toBeVisible();
});
```
