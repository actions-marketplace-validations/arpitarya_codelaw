---
id: testing/playwright-no-wait-for-selector
title: "No `waitForSelector`"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: suggest
fix_reason: "a locator's `waitFor` is strict where `waitForSelector` took the first match"
stability: preview
ported_from: ["eslint-plugin-playwright/no-wait-for-selector"]
since: 0.5.0
---

`page.waitForSelector(selector)` is written `page.locator(selector).waitFor()`; better still, a web-first assertion. The suggested edit makes the first change.

**Why:** `waitForSelector` returns an element handle that goes stale; locators re-find the element.

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
  any:
    - all: [{ pattern: "$P.waitForSelector($S)" }, { pattern: $W0 }]
    - all: [{ pattern: "$P.waitForSelector($S, $O)" }, { pattern: $W1 }]
message: "Use a locator instead of `waitForSelector`"
fix:
  - replace: { target: $W0, with: '$P.locator($S).waitFor()' }
  - replace: { target: $W1, with: '$P.locator($S).waitFor($O)' }
```

## Examples

### JavaScript

#### Don't

```js
test("t", async ({ page }) => {
  await page.waitForSelector(".done");
});
```

#### Do

```js
test("t", async ({ page }) => {
  await page.locator(".done").waitFor();
});
```

### JSX

#### Don't

```jsx
test("t", async ({ page }) => {
  await page.waitForSelector(".done");
});
```

#### Do

```jsx
test("t", async ({ page }) => {
  await page.locator(".done").waitFor();
});
```

### TypeScript

#### Don't

```ts
test("t", async ({ page }) => {
  await page.waitForSelector(".done");
});
```

#### Do

```ts
test("t", async ({ page }) => {
  await page.locator(".done").waitFor();
});
```

### TSX

#### Don't

```tsx
test("t", async ({ page }) => {
  await page.waitForSelector(".done");
});
```

#### Do

```tsx
test("t", async ({ page }) => {
  await page.locator(".done").waitFor();
});
```
