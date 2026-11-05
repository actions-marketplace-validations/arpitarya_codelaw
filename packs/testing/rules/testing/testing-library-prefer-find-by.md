---
id: testing/testing-library-prefer-find-by
title: "`findBy` over `waitFor` + `getBy`"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: auto
stability: preview
ported_from: ["eslint-plugin-testing-library/prefer-find-by"]
since: 0.5.0
---

`waitFor(() => screen.getByText("x"))` is written `screen.findByText("x")` (and `getAllBy`/`queryBy`/`queryAllBy` likewise, with or without `screen.`).

**Why:** `findBy` is exactly `waitFor` plus the query, in one call that reads as such.

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
    - all: [{ pattern: "waitFor(() => screen.getByRole($$$A))" }, { pattern: $W0 }]
    - all: [{ pattern: "waitFor(() => screen.getByLabelText($$$A))" }, { pattern: $W1 }]
    - all: [{ pattern: "waitFor(() => screen.getByPlaceholderText($$$A))" }, { pattern: $W2 }]
    - all: [{ pattern: "waitFor(() => screen.getByText($$$A))" }, { pattern: $W3 }]
    - all: [{ pattern: "waitFor(() => screen.getByDisplayValue($$$A))" }, { pattern: $W4 }]
    - all: [{ pattern: "waitFor(() => screen.getByAltText($$$A))" }, { pattern: $W5 }]
    - all: [{ pattern: "waitFor(() => screen.getByTitle($$$A))" }, { pattern: $W6 }]
    - all: [{ pattern: "waitFor(() => screen.getByTestId($$$A))" }, { pattern: $W7 }]
    - all: [{ pattern: "waitFor(() => screen.getAllByRole($$$A))" }, { pattern: $W8 }]
    - all: [{ pattern: "waitFor(() => screen.getAllByLabelText($$$A))" }, { pattern: $W9 }]
    - all: [{ pattern: "waitFor(() => screen.getAllByPlaceholderText($$$A))" }, { pattern: $W10 }]
    - all: [{ pattern: "waitFor(() => screen.getAllByText($$$A))" }, { pattern: $W11 }]
    - all: [{ pattern: "waitFor(() => screen.getAllByDisplayValue($$$A))" }, { pattern: $W12 }]
    - all: [{ pattern: "waitFor(() => screen.getAllByAltText($$$A))" }, { pattern: $W13 }]
    - all: [{ pattern: "waitFor(() => screen.getAllByTitle($$$A))" }, { pattern: $W14 }]
    - all: [{ pattern: "waitFor(() => screen.getAllByTestId($$$A))" }, { pattern: $W15 }]
    - all: [{ pattern: "waitFor(() => screen.queryByRole($$$A))" }, { pattern: $W16 }]
    - all: [{ pattern: "waitFor(() => screen.queryByLabelText($$$A))" }, { pattern: $W17 }]
    - all: [{ pattern: "waitFor(() => screen.queryByPlaceholderText($$$A))" }, { pattern: $W18 }]
    - all: [{ pattern: "waitFor(() => screen.queryByText($$$A))" }, { pattern: $W19 }]
    - all: [{ pattern: "waitFor(() => screen.queryByDisplayValue($$$A))" }, { pattern: $W20 }]
    - all: [{ pattern: "waitFor(() => screen.queryByAltText($$$A))" }, { pattern: $W21 }]
    - all: [{ pattern: "waitFor(() => screen.queryByTitle($$$A))" }, { pattern: $W22 }]
    - all: [{ pattern: "waitFor(() => screen.queryByTestId($$$A))" }, { pattern: $W23 }]
    - all: [{ pattern: "waitFor(() => screen.queryAllByRole($$$A))" }, { pattern: $W24 }]
    - all: [{ pattern: "waitFor(() => screen.queryAllByLabelText($$$A))" }, { pattern: $W25 }]
    - all: [{ pattern: "waitFor(() => screen.queryAllByPlaceholderText($$$A))" }, { pattern: $W26 }]
    - all: [{ pattern: "waitFor(() => screen.queryAllByText($$$A))" }, { pattern: $W27 }]
    - all: [{ pattern: "waitFor(() => screen.queryAllByDisplayValue($$$A))" }, { pattern: $W28 }]
    - all: [{ pattern: "waitFor(() => screen.queryAllByAltText($$$A))" }, { pattern: $W29 }]
    - all: [{ pattern: "waitFor(() => screen.queryAllByTitle($$$A))" }, { pattern: $W30 }]
    - all: [{ pattern: "waitFor(() => screen.queryAllByTestId($$$A))" }, { pattern: $W31 }]
    - all: [{ pattern: "waitFor(() => getByRole($$$A))" }, { pattern: $W32 }]
    - all: [{ pattern: "waitFor(() => getByLabelText($$$A))" }, { pattern: $W33 }]
    - all: [{ pattern: "waitFor(() => getByPlaceholderText($$$A))" }, { pattern: $W34 }]
    - all: [{ pattern: "waitFor(() => getByText($$$A))" }, { pattern: $W35 }]
    - all: [{ pattern: "waitFor(() => getByDisplayValue($$$A))" }, { pattern: $W36 }]
    - all: [{ pattern: "waitFor(() => getByAltText($$$A))" }, { pattern: $W37 }]
    - all: [{ pattern: "waitFor(() => getByTitle($$$A))" }, { pattern: $W38 }]
    - all: [{ pattern: "waitFor(() => getByTestId($$$A))" }, { pattern: $W39 }]
    - all: [{ pattern: "waitFor(() => getAllByRole($$$A))" }, { pattern: $W40 }]
    - all: [{ pattern: "waitFor(() => getAllByLabelText($$$A))" }, { pattern: $W41 }]
    - all: [{ pattern: "waitFor(() => getAllByPlaceholderText($$$A))" }, { pattern: $W42 }]
    - all: [{ pattern: "waitFor(() => getAllByText($$$A))" }, { pattern: $W43 }]
    - all: [{ pattern: "waitFor(() => getAllByDisplayValue($$$A))" }, { pattern: $W44 }]
    - all: [{ pattern: "waitFor(() => getAllByAltText($$$A))" }, { pattern: $W45 }]
    - all: [{ pattern: "waitFor(() => getAllByTitle($$$A))" }, { pattern: $W46 }]
    - all: [{ pattern: "waitFor(() => getAllByTestId($$$A))" }, { pattern: $W47 }]
    - all: [{ pattern: "waitFor(() => queryByRole($$$A))" }, { pattern: $W48 }]
    - all: [{ pattern: "waitFor(() => queryByLabelText($$$A))" }, { pattern: $W49 }]
    - all: [{ pattern: "waitFor(() => queryByPlaceholderText($$$A))" }, { pattern: $W50 }]
    - all: [{ pattern: "waitFor(() => queryByText($$$A))" }, { pattern: $W51 }]
    - all: [{ pattern: "waitFor(() => queryByDisplayValue($$$A))" }, { pattern: $W52 }]
    - all: [{ pattern: "waitFor(() => queryByAltText($$$A))" }, { pattern: $W53 }]
    - all: [{ pattern: "waitFor(() => queryByTitle($$$A))" }, { pattern: $W54 }]
    - all: [{ pattern: "waitFor(() => queryByTestId($$$A))" }, { pattern: $W55 }]
    - all: [{ pattern: "waitFor(() => queryAllByRole($$$A))" }, { pattern: $W56 }]
    - all: [{ pattern: "waitFor(() => queryAllByLabelText($$$A))" }, { pattern: $W57 }]
    - all: [{ pattern: "waitFor(() => queryAllByPlaceholderText($$$A))" }, { pattern: $W58 }]
    - all: [{ pattern: "waitFor(() => queryAllByText($$$A))" }, { pattern: $W59 }]
    - all: [{ pattern: "waitFor(() => queryAllByDisplayValue($$$A))" }, { pattern: $W60 }]
    - all: [{ pattern: "waitFor(() => queryAllByAltText($$$A))" }, { pattern: $W61 }]
    - all: [{ pattern: "waitFor(() => queryAllByTitle($$$A))" }, { pattern: $W62 }]
    - all: [{ pattern: "waitFor(() => queryAllByTestId($$$A))" }, { pattern: $W63 }]
message: "Use a `findBy` query"
fix:
  - replace: { target: $W0, with: 'screen.findByRole($$$A)' }
  - replace: { target: $W1, with: 'screen.findByLabelText($$$A)' }
  - replace: { target: $W2, with: 'screen.findByPlaceholderText($$$A)' }
  - replace: { target: $W3, with: 'screen.findByText($$$A)' }
  - replace: { target: $W4, with: 'screen.findByDisplayValue($$$A)' }
  - replace: { target: $W5, with: 'screen.findByAltText($$$A)' }
  - replace: { target: $W6, with: 'screen.findByTitle($$$A)' }
  - replace: { target: $W7, with: 'screen.findByTestId($$$A)' }
  - replace: { target: $W8, with: 'screen.findAllByRole($$$A)' }
  - replace: { target: $W9, with: 'screen.findAllByLabelText($$$A)' }
  - replace: { target: $W10, with: 'screen.findAllByPlaceholderText($$$A)' }
  - replace: { target: $W11, with: 'screen.findAllByText($$$A)' }
  - replace: { target: $W12, with: 'screen.findAllByDisplayValue($$$A)' }
  - replace: { target: $W13, with: 'screen.findAllByAltText($$$A)' }
  - replace: { target: $W14, with: 'screen.findAllByTitle($$$A)' }
  - replace: { target: $W15, with: 'screen.findAllByTestId($$$A)' }
  - replace: { target: $W16, with: 'screen.findByRole($$$A)' }
  - replace: { target: $W17, with: 'screen.findByLabelText($$$A)' }
  - replace: { target: $W18, with: 'screen.findByPlaceholderText($$$A)' }
  - replace: { target: $W19, with: 'screen.findByText($$$A)' }
  - replace: { target: $W20, with: 'screen.findByDisplayValue($$$A)' }
  - replace: { target: $W21, with: 'screen.findByAltText($$$A)' }
  - replace: { target: $W22, with: 'screen.findByTitle($$$A)' }
  - replace: { target: $W23, with: 'screen.findByTestId($$$A)' }
  - replace: { target: $W24, with: 'screen.findAllByRole($$$A)' }
  - replace: { target: $W25, with: 'screen.findAllByLabelText($$$A)' }
  - replace: { target: $W26, with: 'screen.findAllByPlaceholderText($$$A)' }
  - replace: { target: $W27, with: 'screen.findAllByText($$$A)' }
  - replace: { target: $W28, with: 'screen.findAllByDisplayValue($$$A)' }
  - replace: { target: $W29, with: 'screen.findAllByAltText($$$A)' }
  - replace: { target: $W30, with: 'screen.findAllByTitle($$$A)' }
  - replace: { target: $W31, with: 'screen.findAllByTestId($$$A)' }
  - replace: { target: $W32, with: 'findByRole($$$A)' }
  - replace: { target: $W33, with: 'findByLabelText($$$A)' }
  - replace: { target: $W34, with: 'findByPlaceholderText($$$A)' }
  - replace: { target: $W35, with: 'findByText($$$A)' }
  - replace: { target: $W36, with: 'findByDisplayValue($$$A)' }
  - replace: { target: $W37, with: 'findByAltText($$$A)' }
  - replace: { target: $W38, with: 'findByTitle($$$A)' }
  - replace: { target: $W39, with: 'findByTestId($$$A)' }
  - replace: { target: $W40, with: 'findAllByRole($$$A)' }
  - replace: { target: $W41, with: 'findAllByLabelText($$$A)' }
  - replace: { target: $W42, with: 'findAllByPlaceholderText($$$A)' }
  - replace: { target: $W43, with: 'findAllByText($$$A)' }
  - replace: { target: $W44, with: 'findAllByDisplayValue($$$A)' }
  - replace: { target: $W45, with: 'findAllByAltText($$$A)' }
  - replace: { target: $W46, with: 'findAllByTitle($$$A)' }
  - replace: { target: $W47, with: 'findAllByTestId($$$A)' }
  - replace: { target: $W48, with: 'findByRole($$$A)' }
  - replace: { target: $W49, with: 'findByLabelText($$$A)' }
  - replace: { target: $W50, with: 'findByPlaceholderText($$$A)' }
  - replace: { target: $W51, with: 'findByText($$$A)' }
  - replace: { target: $W52, with: 'findByDisplayValue($$$A)' }
  - replace: { target: $W53, with: 'findByAltText($$$A)' }
  - replace: { target: $W54, with: 'findByTitle($$$A)' }
  - replace: { target: $W55, with: 'findByTestId($$$A)' }
  - replace: { target: $W56, with: 'findAllByRole($$$A)' }
  - replace: { target: $W57, with: 'findAllByLabelText($$$A)' }
  - replace: { target: $W58, with: 'findAllByPlaceholderText($$$A)' }
  - replace: { target: $W59, with: 'findAllByText($$$A)' }
  - replace: { target: $W60, with: 'findAllByDisplayValue($$$A)' }
  - replace: { target: $W61, with: 'findAllByAltText($$$A)' }
  - replace: { target: $W62, with: 'findAllByTitle($$$A)' }
  - replace: { target: $W63, with: 'findAllByTestId($$$A)' }
```

## Examples

### JavaScript

#### Don't

```js
test("t", async () => {
  const t = await waitFor(() => screen.getByText("Hi"));
});
```

#### Do

```js
test("t", async () => {
  const t = await screen.findByText("Hi");
});
```

### JSX

#### Don't

```jsx
test("t", async () => {
  const t = await waitFor(() => screen.getByText("Hi"));
});
```

#### Do

```jsx
test("t", async () => {
  const t = await screen.findByText("Hi");
});
```

### TypeScript

#### Don't

```ts
test("t", async () => {
  const t = await waitFor(() => screen.getByText("Hi"));
});
```

#### Do

```ts
test("t", async () => {
  const t = await screen.findByText("Hi");
});
```

### TSX

#### Don't

```tsx
test("t", async () => {
  const t = await waitFor(() => screen.getByText("Hi"));
});
```

#### Do

```tsx
test("t", async () => {
  const t = await screen.findByText("Hi");
});
```
