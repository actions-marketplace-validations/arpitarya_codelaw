---
id: testing/playwright-no-useless-not
title: "No `.not` where a matcher says it"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: auto
stability: preview
ported_from: ["eslint-plugin-playwright/no-useless-not"]
since: 0.5.0
---

`not.toBeVisible()` is `toBeHidden()`, `not.toBeHidden()` is `toBeVisible()`, `not.toBeEnabled()` is `toBeDisabled()` and `not.toBeDisabled()` is `toBeEnabled()`; the fix uses the positive matcher.

**Why:** The positive matcher reads directly and waits for the state it names.

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
    - all: [{ pattern: "expect($A).not.toBeVisible($$$R)" }, { pattern: $W0 }]
    - all: [{ pattern: "expect($A).not.toBeHidden($$$R)" }, { pattern: $W1 }]
    - all: [{ pattern: "expect($A).not.toBeEnabled($$$R)" }, { pattern: $W2 }]
    - all: [{ pattern: "expect($A).not.toBeDisabled($$$R)" }, { pattern: $W3 }]
message: "Use the matcher for this state"
fix:
  - replace: { target: $W0, with: 'expect($A).toBeHidden($$$R)' }
  - replace: { target: $W1, with: 'expect($A).toBeVisible($$$R)' }
  - replace: { target: $W2, with: 'expect($A).toBeDisabled($$$R)' }
  - replace: { target: $W3, with: 'expect($A).toBeEnabled($$$R)' }
```

## Examples

### JavaScript

#### Don't

```js
test("t", async ({ page }) => {
  await expect(page.getByText("Busy")).not.toBeVisible();
});
```

#### Do

```js
test("t", async ({ page }) => {
  await expect(page.getByText("Busy")).toBeHidden();
});
```

### JSX

#### Don't

```jsx
test("t", async ({ page }) => {
  await expect(page.getByText("Busy")).not.toBeVisible();
});
```

#### Do

```jsx
test("t", async ({ page }) => {
  await expect(page.getByText("Busy")).toBeHidden();
});
```

### TypeScript

#### Don't

```ts
test("t", async ({ page }) => {
  await expect(page.getByText("Busy")).not.toBeVisible();
});
```

#### Do

```ts
test("t", async ({ page }) => {
  await expect(page.getByText("Busy")).toBeHidden();
});
```

### TSX

#### Don't

```tsx
test("t", async ({ page }) => {
  await expect(page.getByText("Busy")).not.toBeVisible();
});
```

#### Do

```tsx
test("t", async ({ page }) => {
  await expect(page.getByText("Busy")).toBeHidden();
});
```
