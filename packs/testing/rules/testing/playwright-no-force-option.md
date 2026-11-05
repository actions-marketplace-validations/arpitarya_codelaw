---
id: testing/playwright-no-force-option
title: "No `force: true` on actions"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: suggest
fix_reason: "without `force` the action waits for the element to be actionable, and may now time out"
stability: preview
ported_from: ["eslint-plugin-playwright/no-force-option"]
since: 0.5.0
---

An action is not given `force: true`. The suggested edit removes the option.

**Why:** `force` skips the actionability checks — visible, enabled, not covered — so the test clicks what a user could not.

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
  kind: pair
  all:
    - has: { field: key, regex: '^force$' }
    - has: { field: value, kind: 'true' }
    - inside: { kind: object, inside: { kind: arguments, inside: { kind: call_expression, has: { field: function, kind: member_expression, has: { field: property, regex: '^(click|dblclick|fill|type|press|pressSequentially|check|uncheck|setChecked|hover|focus|tap|selectOption|setInputFiles|dragTo|clear|blur|selectText|scrollIntoViewIfNeeded|dispatchEvent)$' } } } } }
message: "Do not force the action"
fix: [remove]
```

## Examples

### JavaScript

#### Don't

```js
test("t", async ({ page }) => {
  await page.getByRole("button").click({ force: true, delay: 1 });
});
```

#### Do

```js
test("t", async ({ page }) => {
  await page.getByRole("button").click({ delay: 1 });
});
```

### JSX

#### Don't

```jsx
test("t", async ({ page }) => {
  await page.getByRole("button").click({ force: true, delay: 1 });
});
```

#### Do

```jsx
test("t", async ({ page }) => {
  await page.getByRole("button").click({ delay: 1 });
});
```

### TypeScript

#### Don't

```ts
test("t", async ({ page }) => {
  await page.getByRole("button").click({ force: true, delay: 1 });
});
```

#### Do

```ts
test("t", async ({ page }) => {
  await page.getByRole("button").click({ delay: 1 });
});
```

### TSX

#### Don't

```tsx
test("t", async ({ page }) => {
  await page.getByRole("button").click({ force: true, delay: 1 });
});
```

#### Do

```tsx
test("t", async ({ page }) => {
  await page.getByRole("button").click({ delay: 1 });
});
```
