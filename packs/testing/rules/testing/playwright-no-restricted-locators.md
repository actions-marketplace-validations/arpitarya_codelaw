---
id: testing/playwright-no-restricted-locators
title: "No locator methods the project restricts"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: manual
fix_reason: "which locator to use instead is the project's convention"
fix_exception: value
stability: preview
options: { locators: { type: "string[]", default: [] } }
ported_from: ["eslint-plugin-playwright/no-restricted-locators"]
since: 0.5.0
---

A locator method the project lists in `locators` is reported: `locators = ["getByTestId", "locator"]`. The list is empty by default.

**Why:** A project restricts a locator when it has chosen a more user-facing one.

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
  has: { field: function, kind: member_expression, has: { field: property, pattern: $L } }
where:
  $L: { in: locators }
message: "`$L` is restricted in this project"
```

## How to fix

1. Use the locator the project prefers, usually `getByRole` or `getByLabel`.

## Examples

### JavaScript

#### Don't

```js options={"locators":["getByTestId"]}
test("t", async ({ page }) => {
  await page.getByTestId("save").click();
});
```

#### Do

```js options={"locators":["getByTestId"]}
test("t", async ({ page }) => {
  await page.getByRole("button", { name: "Save" }).click();
});
```

### JSX

#### Don't

```jsx options={"locators":["getByTestId"]}
test("t", async ({ page }) => {
  await page.getByTestId("save").click();
});
```

#### Do

```jsx options={"locators":["getByTestId"]}
test("t", async ({ page }) => {
  await page.getByRole("button", { name: "Save" }).click();
});
```

### TypeScript

#### Don't

```ts options={"locators":["getByTestId"]}
test("t", async ({ page }) => {
  await page.getByTestId("save").click();
});
```

#### Do

```ts options={"locators":["getByTestId"]}
test("t", async ({ page }) => {
  await page.getByRole("button", { name: "Save" }).click();
});
```

### TSX

#### Don't

```tsx options={"locators":["getByTestId"]}
test("t", async ({ page }) => {
  await page.getByTestId("save").click();
});
```

#### Do

```tsx options={"locators":["getByTestId"]}
test("t", async ({ page }) => {
  await page.getByRole("button", { name: "Save" }).click();
});
```
