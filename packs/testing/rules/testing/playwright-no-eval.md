---
id: testing/playwright-no-eval
title: "No `$eval` and `$$eval`"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: manual
fix_reason: "the evaluation becomes a locator assertion or `locator.evaluate`, which needs knowing what is checked"
fix_exception: restructure
stability: preview
ported_from: ["eslint-plugin-playwright/no-eval"]
since: 0.5.0
---

A test does not call `page.$eval` or `page.$$eval`; it uses a locator (`locator.evaluate`, or a web-first assertion).

**Why:** `$eval` runs once against whatever matches at that instant, without waiting; a locator waits and retries.

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
  has: { field: function, kind: member_expression, has: { field: property, regex: '^\$\$?eval$' } }
message: "Use a locator instead of `$eval`"
```

## How to fix

1. Replace `page.$eval(sel, fn)` with `page.locator(sel).evaluate(fn)`, or with a web-first assertion on the locator.

## Examples

### JavaScript

#### Don't

```js
test("t", async ({ page }) => {
  await page.$eval("h1", (e) => e.textContent);
});
```

#### Do

```js
test("t", async ({ page }) => {
  await expect(page.locator("h1")).toHaveText("Hi");
});
```

### JSX

#### Don't

```jsx
test("t", async ({ page }) => {
  await page.$eval("h1", (e) => e.textContent);
});
```

#### Do

```jsx
test("t", async ({ page }) => {
  await expect(page.locator("h1")).toHaveText("Hi");
});
```

### TypeScript

#### Don't

```ts
test("t", async ({ page }) => {
  await page.$eval("h1", (e) => e.textContent);
});
```

#### Do

```ts
test("t", async ({ page }) => {
  await expect(page.locator("h1")).toHaveText("Hi");
});
```

### TSX

#### Don't

```tsx
test("t", async ({ page }) => {
  await page.$eval("h1", (e) => e.textContent);
});
```

#### Do

```tsx
test("t", async ({ page }) => {
  await expect(page.locator("h1")).toHaveText("Hi");
});
```
