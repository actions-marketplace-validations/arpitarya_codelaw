---
id: testing/playwright-prefer-to-have-count
title: "`toHaveCount` for a locator's count"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: suggest
fix_reason: "`toHaveCount` retries until the count matches, so an assertion that failed at once may now pass after waiting"
stability: preview
ported_from: ["eslint-plugin-playwright/prefer-to-have-count"]
since: 0.5.0
---

`expect(await locator.count()).toBe(n)` is written `await expect(locator).toHaveCount(n)`. The suggested edit makes the change.

**Why:** The web-first matcher waits for the count; reading `count()` once races the page.

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
    - all: [{ pattern: "expect(await $L.count()).toBe($N)" }, { pattern: $W0 }]
    - all: [{ pattern: "expect(await $L.count()).toEqual($N)" }, { pattern: $W1 }]
    - all: [{ pattern: "expect(await $L.count()).not.toBe($N)" }, { pattern: $W2 }]
message: "Use `toHaveCount`"
fix:
  - replace: { target: $W0, with: 'await expect($L).toHaveCount($N)' }
  - replace: { target: $W1, with: 'await expect($L).toHaveCount($N)' }
  - replace: { target: $W2, with: 'await expect($L).not.toHaveCount($N)' }
```

## Examples

### JavaScript

#### Don't

```js
test("t", async ({ page }) => {
  expect(await page.getByRole("row").count()).toBe(3);
});
```

#### Do

```js
test("t", async ({ page }) => {
  await expect(page.getByRole("row")).toHaveCount(3);
});
```

### JSX

#### Don't

```jsx
test("t", async ({ page }) => {
  expect(await page.getByRole("row").count()).toBe(3);
});
```

#### Do

```jsx
test("t", async ({ page }) => {
  await expect(page.getByRole("row")).toHaveCount(3);
});
```

### TypeScript

#### Don't

```ts
test("t", async ({ page }) => {
  expect(await page.getByRole("row").count()).toBe(3);
});
```

#### Do

```ts
test("t", async ({ page }) => {
  await expect(page.getByRole("row")).toHaveCount(3);
});
```

### TSX

#### Don't

```tsx
test("t", async ({ page }) => {
  expect(await page.getByRole("row").count()).toBe(3);
});
```

#### Do

```tsx
test("t", async ({ page }) => {
  await expect(page.getByRole("row")).toHaveCount(3);
});
```
