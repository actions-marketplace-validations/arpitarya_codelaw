---
id: testing/testing-library-prefer-query-by-disappearance
title: "`queryBy` in `waitForElementToBeRemoved`"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: suggest
fix_reason: "a `queryBy` callback returns `null` where `getBy` threw; the author checks the test still waits for the right thing"
stability: preview
ported_from: ["eslint-plugin-testing-library/prefer-query-by-disappearance", "eslint-plugin-sonarjs/testing-library-prefer-query-by-disappearance"]
since: 0.5.0
---

`waitForElementToBeRemoved(() => screen.getByText("x"))` uses `queryByText`; the suggested edit swaps the query.

**Why:** A `getBy` callback throws once the element is gone, which ends the wait with an error path instead of the removal it is waiting for.

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
    - all: [{ pattern: "waitForElementToBeRemoved(() => screen.getByRole($$$A))" }, { pattern: $W0 }]
    - all: [{ pattern: "waitForElementToBeRemoved(() => screen.getByLabelText($$$A))" }, { pattern: $W1 }]
    - all: [{ pattern: "waitForElementToBeRemoved(() => screen.getByPlaceholderText($$$A))" }, { pattern: $W2 }]
    - all: [{ pattern: "waitForElementToBeRemoved(() => screen.getByText($$$A))" }, { pattern: $W3 }]
    - all: [{ pattern: "waitForElementToBeRemoved(() => screen.getByDisplayValue($$$A))" }, { pattern: $W4 }]
    - all: [{ pattern: "waitForElementToBeRemoved(() => screen.getByAltText($$$A))" }, { pattern: $W5 }]
    - all: [{ pattern: "waitForElementToBeRemoved(() => screen.getByTitle($$$A))" }, { pattern: $W6 }]
    - all: [{ pattern: "waitForElementToBeRemoved(() => screen.getByTestId($$$A))" }, { pattern: $W7 }]
    - all: [{ pattern: "waitForElementToBeRemoved(() => screen.getAllByRole($$$A))" }, { pattern: $W8 }]
    - all: [{ pattern: "waitForElementToBeRemoved(() => screen.getAllByLabelText($$$A))" }, { pattern: $W9 }]
    - all: [{ pattern: "waitForElementToBeRemoved(() => screen.getAllByPlaceholderText($$$A))" }, { pattern: $W10 }]
    - all: [{ pattern: "waitForElementToBeRemoved(() => screen.getAllByText($$$A))" }, { pattern: $W11 }]
    - all: [{ pattern: "waitForElementToBeRemoved(() => screen.getAllByDisplayValue($$$A))" }, { pattern: $W12 }]
    - all: [{ pattern: "waitForElementToBeRemoved(() => screen.getAllByAltText($$$A))" }, { pattern: $W13 }]
    - all: [{ pattern: "waitForElementToBeRemoved(() => screen.getAllByTitle($$$A))" }, { pattern: $W14 }]
    - all: [{ pattern: "waitForElementToBeRemoved(() => screen.getAllByTestId($$$A))" }, { pattern: $W15 }]
message: "Use `queryBy` in `waitForElementToBeRemoved`"
fix:
  - replace: { target: $W0, with: 'waitForElementToBeRemoved(() => screen.queryByRole($$$A))' }
  - replace: { target: $W1, with: 'waitForElementToBeRemoved(() => screen.queryByLabelText($$$A))' }
  - replace: { target: $W2, with: 'waitForElementToBeRemoved(() => screen.queryByPlaceholderText($$$A))' }
  - replace: { target: $W3, with: 'waitForElementToBeRemoved(() => screen.queryByText($$$A))' }
  - replace: { target: $W4, with: 'waitForElementToBeRemoved(() => screen.queryByDisplayValue($$$A))' }
  - replace: { target: $W5, with: 'waitForElementToBeRemoved(() => screen.queryByAltText($$$A))' }
  - replace: { target: $W6, with: 'waitForElementToBeRemoved(() => screen.queryByTitle($$$A))' }
  - replace: { target: $W7, with: 'waitForElementToBeRemoved(() => screen.queryByTestId($$$A))' }
  - replace: { target: $W8, with: 'waitForElementToBeRemoved(() => screen.queryAllByRole($$$A))' }
  - replace: { target: $W9, with: 'waitForElementToBeRemoved(() => screen.queryAllByLabelText($$$A))' }
  - replace: { target: $W10, with: 'waitForElementToBeRemoved(() => screen.queryAllByPlaceholderText($$$A))' }
  - replace: { target: $W11, with: 'waitForElementToBeRemoved(() => screen.queryAllByText($$$A))' }
  - replace: { target: $W12, with: 'waitForElementToBeRemoved(() => screen.queryAllByDisplayValue($$$A))' }
  - replace: { target: $W13, with: 'waitForElementToBeRemoved(() => screen.queryAllByAltText($$$A))' }
  - replace: { target: $W14, with: 'waitForElementToBeRemoved(() => screen.queryAllByTitle($$$A))' }
  - replace: { target: $W15, with: 'waitForElementToBeRemoved(() => screen.queryAllByTestId($$$A))' }
```

## Examples

### JavaScript

#### Don't

```js
test("t", async () => {
  await waitForElementToBeRemoved(() => screen.getByText("Loading"));
});
```

#### Do

```js
test("t", async () => {
  await waitForElementToBeRemoved(() => screen.queryByText("Loading"));
});
```

### JSX

#### Don't

```jsx
test("t", async () => {
  await waitForElementToBeRemoved(() => screen.getByText("Loading"));
});
```

#### Do

```jsx
test("t", async () => {
  await waitForElementToBeRemoved(() => screen.queryByText("Loading"));
});
```

### TypeScript

#### Don't

```ts
test("t", async () => {
  await waitForElementToBeRemoved(() => screen.getByText("Loading"));
});
```

#### Do

```ts
test("t", async () => {
  await waitForElementToBeRemoved(() => screen.queryByText("Loading"));
});
```

### TSX

#### Don't

```tsx
test("t", async () => {
  await waitForElementToBeRemoved(() => screen.getByText("Loading"));
});
```

#### Do

```tsx
test("t", async () => {
  await waitForElementToBeRemoved(() => screen.queryByText("Loading"));
});
```
