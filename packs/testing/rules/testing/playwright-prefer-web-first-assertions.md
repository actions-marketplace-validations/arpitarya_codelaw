---
id: testing/playwright-prefer-web-first-assertions
title: "Web-first assertions over reading state"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: suggest
fix_reason: "the web-first matcher retries until it passes, so an assertion that failed at once may now pass after waiting"
stability: preview
ported_from: ["eslint-plugin-playwright/prefer-web-first-assertions"]
since: 0.5.0
---

`expect(await locator.isVisible()).toBe(true)` is written `await expect(locator).toBeVisible()`; likewise `isHidden`, `isEnabled`, `isDisabled`, `isChecked` and `isEditable`, `textContent()`/`innerText()` (`toHaveText`) and `inputValue()` (`toHaveValue`). The suggested edit makes the change.

**Why:** Reading the state once races the page; a web-first assertion waits for it.

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
    - all: [{ pattern: "expect(await $L.isVisible()).toBe(true)" }, { pattern: $W0 }]
    - all: [{ pattern: "expect(await $L.isHidden()).toBe(true)" }, { pattern: $W1 }]
    - all: [{ pattern: "expect(await $L.isEnabled()).toBe(true)" }, { pattern: $W2 }]
    - all: [{ pattern: "expect(await $L.isDisabled()).toBe(true)" }, { pattern: $W3 }]
    - all: [{ pattern: "expect(await $L.isChecked()).toBe(true)" }, { pattern: $W4 }]
    - all: [{ pattern: "expect(await $L.isEditable()).toBe(true)" }, { pattern: $W5 }]
    - all: [{ pattern: "expect(await $L.isVisible()).toBe(false)" }, { pattern: $W6 }]
    - all: [{ pattern: "expect(await $L.isHidden()).toBe(false)" }, { pattern: $W7 }]
    - all: [{ pattern: "expect(await $L.isEnabled()).toBe(false)" }, { pattern: $W8 }]
    - all: [{ pattern: "expect(await $L.isDisabled()).toBe(false)" }, { pattern: $W9 }]
    - all: [{ pattern: "expect(await $L.isChecked()).toBe(false)" }, { pattern: $W10 }]
    - all: [{ pattern: "expect(await $L.isEditable()).toBe(false)" }, { pattern: $W11 }]
    - all: [{ pattern: "expect(await $L.textContent()).toBe($V)" }, { pattern: $W12 }]
    - all: [{ pattern: "expect(await $L.innerText()).toBe($V)" }, { pattern: $W13 }]
    - all: [{ pattern: "expect(await $L.inputValue()).toBe($V)" }, { pattern: $W14 }]
message: "Use a web-first assertion"
fix:
  - replace: { target: $W0, with: 'await expect($L).toBeVisible()' }
  - replace: { target: $W1, with: 'await expect($L).toBeHidden()' }
  - replace: { target: $W2, with: 'await expect($L).toBeEnabled()' }
  - replace: { target: $W3, with: 'await expect($L).toBeDisabled()' }
  - replace: { target: $W4, with: 'await expect($L).toBeChecked()' }
  - replace: { target: $W5, with: 'await expect($L).toBeEditable()' }
  - replace: { target: $W6, with: 'await expect($L).not.toBeVisible()' }
  - replace: { target: $W7, with: 'await expect($L).not.toBeHidden()' }
  - replace: { target: $W8, with: 'await expect($L).not.toBeEnabled()' }
  - replace: { target: $W9, with: 'await expect($L).not.toBeDisabled()' }
  - replace: { target: $W10, with: 'await expect($L).not.toBeChecked()' }
  - replace: { target: $W11, with: 'await expect($L).not.toBeEditable()' }
  - replace: { target: $W12, with: 'await expect($L).toHaveText($V)' }
  - replace: { target: $W13, with: 'await expect($L).toHaveText($V)' }
  - replace: { target: $W14, with: 'await expect($L).toHaveValue($V)' }
```

## Examples

### JavaScript

#### Don't

```js
test("t", async ({ page }) => {
  expect(await page.getByText("Saved").isVisible()).toBe(true);
});
```

#### Do

```js
test("t", async ({ page }) => {
  await expect(page.getByText("Saved")).toBeVisible();
});
```

### JSX

#### Don't

```jsx
test("t", async ({ page }) => {
  expect(await page.getByText("Saved").isVisible()).toBe(true);
});
```

#### Do

```jsx
test("t", async ({ page }) => {
  await expect(page.getByText("Saved")).toBeVisible();
});
```

### TypeScript

#### Don't

```ts
test("t", async ({ page }) => {
  expect(await page.getByText("Saved").isVisible()).toBe(true);
});
```

#### Do

```ts
test("t", async ({ page }) => {
  await expect(page.getByText("Saved")).toBeVisible();
});
```

### TSX

#### Don't

```tsx
test("t", async ({ page }) => {
  expect(await page.getByText("Saved").isVisible()).toBe(true);
});
```

#### Do

```tsx
test("t", async ({ page }) => {
  await expect(page.getByText("Saved")).toBeVisible();
});
```
