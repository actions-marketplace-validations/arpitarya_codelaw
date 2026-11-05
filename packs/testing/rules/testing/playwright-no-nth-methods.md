---
id: testing/playwright-no-nth-methods
title: "No `first()`, `last()` and `nth()`"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: manual
fix_reason: "a unique locator needs knowing what distinguishes the element"
fix_exception: restructure
stability: preview
ported_from: ["eslint-plugin-playwright/no-nth-methods"]
since: 0.5.0
---

A locator is made unique by what it matches, not narrowed with `.first()`, `.last()` or `.nth(i)`. Opt-in.

**Why:** Position-based locators break when the order of the page changes, and pick the wrong element silently.

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
    - has: { field: function, kind: member_expression, has: { field: property, regex: '^(first|last|nth)$' } }
    - has: { field: function, has: { field: object, kind: call_expression } }
message: "Locate the element by what it is, not its position"
```

## How to fix

1. Add the distinguishing text, name or role to the locator, or `filter({ hasText })`.

## Examples

### JavaScript

#### Don't

```js
test("t", async ({ page }) => {
  await page.getByRole("button").first().click();
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
  await page.getByRole("button").first().click();
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
  await page.getByRole("button").first().click();
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
  await page.getByRole("button").first().click();
});
```

#### Do

```tsx
test("t", async ({ page }) => {
  await page.getByRole("button", { name: "Save" }).click();
});
```
