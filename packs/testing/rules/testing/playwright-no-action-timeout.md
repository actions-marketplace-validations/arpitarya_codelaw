---
id: testing/playwright-no-action-timeout
title: "No `timeout` on an action"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: suggest
fix_reason: "dropping the timeout makes the action use the configured one, which may be shorter"
stability: preview
ported_from: ["eslint-plugin-playwright/no-action-timeout"]
since: 0.5.0
---

An action (`click`, `fill`, `press`, …) is not given its own `timeout`; the project's `actionTimeout` applies. The suggested edit removes the option.

**Why:** A per-call timeout hides a slow page behind one long wait and drifts from the configured limit.

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
    - has: { field: key, regex: '^timeout$' }
    - has: { field: value, kind: number }
    - inside: { kind: object, inside: { kind: arguments, inside: { kind: call_expression, has: { field: function, kind: member_expression, has: { field: property, regex: '^(click|dblclick|fill|type|press|pressSequentially|check|uncheck|setChecked|hover|focus|tap|selectOption|setInputFiles|dragTo|clear|blur|selectText|scrollIntoViewIfNeeded|dispatchEvent)$' } } } } }
message: "Use the configured action timeout"
fix: [remove]
```

## Examples

### JavaScript

#### Don't

```js
test("t", async ({ page }) => {
  await page.getByRole("button").click({ timeout: 10000, force: false });
});
```

#### Do

```js
test("t", async ({ page }) => {
  await page.getByRole("button").click({ force: false });
});
```

### JSX

#### Don't

```jsx
test("t", async ({ page }) => {
  await page.getByRole("button").click({ timeout: 10000, force: false });
});
```

#### Do

```jsx
test("t", async ({ page }) => {
  await page.getByRole("button").click({ force: false });
});
```

### TypeScript

#### Don't

```ts
test("t", async ({ page }) => {
  await page.getByRole("button").click({ timeout: 10000, force: false });
});
```

#### Do

```ts
test("t", async ({ page }) => {
  await page.getByRole("button").click({ force: false });
});
```

### TSX

#### Don't

```tsx
test("t", async ({ page }) => {
  await page.getByRole("button").click({ timeout: 10000, force: false });
});
```

#### Do

```tsx
test("t", async ({ page }) => {
  await page.getByRole("button").click({ force: false });
});
```
