---
id: testing/playwright-no-element-handle
title: "Locators, not element handles"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: suggest
fix_reason: "a locator is lazy and strict where a handle was resolved once, so later lines may behave differently"
stability: preview
ported_from: ["eslint-plugin-playwright/no-element-handle"]
since: 0.5.0
---

`page.$(selector)` and `page.$$(selector)` are written `page.locator(selector)`. The suggested edit makes the change.

**Why:** An element handle points at one DOM node, which goes stale when the page re-renders; a locator finds the element again each time and auto-waits.

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
    - all: [{ pattern: "$P.$($S)" }, { pattern: $W0 }]
    - all: [{ pattern: "$P.$$($S)" }, { pattern: $W1 }]
message: "Use a locator instead of an element handle"
fix:
  - replace: { target: $W0, with: '$P.locator($S)' }
  - replace: { target: $W1, with: '$P.locator($S)' }
```

## Examples

### JavaScript

#### Don't

```js
test("t", async ({ page }) => {
  const b = await page.$("button");
});
```

#### Do

```js
test("t", async ({ page }) => {
  const b = await page.locator("button");
});
```

### JSX

#### Don't

```jsx
test("t", async ({ page }) => {
  const b = await page.$("button");
});
```

#### Do

```jsx
test("t", async ({ page }) => {
  const b = await page.locator("button");
});
```

### TypeScript

#### Don't

```ts
test("t", async ({ page }) => {
  const b = await page.$("button");
});
```

#### Do

```ts
test("t", async ({ page }) => {
  const b = await page.locator("button");
});
```

### TSX

#### Don't

```tsx
test("t", async ({ page }) => {
  const b = await page.$("button");
});
```

#### Do

```tsx
test("t", async ({ page }) => {
  const b = await page.locator("button");
});
```
