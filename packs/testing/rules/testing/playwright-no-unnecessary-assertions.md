---
id: testing/playwright-no-unnecessary-assertions
title: "No assertion on a constant"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: suggest
fix_reason: "deleting the assertion is safe only if it was indeed left in by mistake"
stability: preview
ported_from: ["eslint-plugin-playwright/no-unnecessary-assertions"]
since: 0.5.0
---

`expect` is not given a literal (`expect(true).toBe(true)`, `expect(1).toBeTruthy()`): such an assertion always passes or always fails. The suggested edit deletes it.

**Why:** An assertion on a constant checks nothing about the page.

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
  has:
    kind: call_expression
    has:
      field: function
      kind: member_expression
      has:
        field: object
        matches: expect-call
        has: { field: arguments, all: [{ has: { nthChild: 1, any: [{ kind: number }, { kind: string }, { kind: 'true' }, { kind: 'false' }, { kind: 'null' }] } }, { not: { has: { nthChild: 2 } } }] }
message: "This assertion checks a constant"
fix: [remove]
```

## Examples

### JavaScript

#### Don't

```js
test("t", async ({ page }) => {
  expect(true).toBe(true);
  await expect(page).toHaveTitle("Home");
});
```

#### Do

```js
test("t", async ({ page }) => {
  await expect(page).toHaveTitle("Home");
});
```

### JSX

#### Don't

```jsx
test("t", async ({ page }) => {
  expect(true).toBe(true);
  await expect(page).toHaveTitle("Home");
});
```

#### Do

```jsx
test("t", async ({ page }) => {
  await expect(page).toHaveTitle("Home");
});
```

### TypeScript

#### Don't

```ts
test("t", async ({ page }) => {
  expect(true).toBe(true);
  await expect(page).toHaveTitle("Home");
});
```

#### Do

```ts
test("t", async ({ page }) => {
  await expect(page).toHaveTitle("Home");
});
```

### TSX

#### Don't

```tsx
test("t", async ({ page }) => {
  expect(true).toBe(true);
  await expect(page).toHaveTitle("Home");
});
```

#### Do

```tsx
test("t", async ({ page }) => {
  await expect(page).toHaveTitle("Home");
});
```
