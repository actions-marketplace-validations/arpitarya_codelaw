---
id: testing/playwright-require-soft-assertions
title: "Soft assertions"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: suggest
fix_reason: "a soft assertion lets the test go on after a failure, which changes what runs after it"
stability: preview
ported_from: ["eslint-plugin-playwright/require-soft-assertions"]
since: 0.5.0
---

Assertions use `expect.soft(…)`, so a test reports every failure, not only the first. The suggested edit changes `expect` to `expect.soft`. Opt-in.

**Why:** In a long end-to-end test, stopping at the first failed check hides the others.

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
  kind: identifier
  regex: '^expect$'
  inside: { kind: call_expression, field: function, inside: { kind: member_expression, field: object, inside: { kind: call_expression, field: function } } }
message: "Use `expect.soft`"
fix: expect.soft
```

## Examples

### JavaScript

#### Don't

```js
test("t", async ({ page }) => {
  await expect(page).toHaveTitle("Home");
});
```

#### Do

```js
test("t", async ({ page }) => {
  await expect.soft(page).toHaveTitle("Home");
});
```

### JSX

#### Don't

```jsx
test("t", async ({ page }) => {
  await expect(page).toHaveTitle("Home");
});
```

#### Do

```jsx
test("t", async ({ page }) => {
  await expect.soft(page).toHaveTitle("Home");
});
```

### TypeScript

#### Don't

```ts
test("t", async ({ page }) => {
  await expect(page).toHaveTitle("Home");
});
```

#### Do

```ts
test("t", async ({ page }) => {
  await expect.soft(page).toHaveTitle("Home");
});
```

### TSX

#### Don't

```tsx
test("t", async ({ page }) => {
  await expect(page).toHaveTitle("Home");
});
```

#### Do

```tsx
test("t", async ({ page }) => {
  await expect.soft(page).toHaveTitle("Home");
});
```
