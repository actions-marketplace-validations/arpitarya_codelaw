---
id: testing/testing-library-prefer-presence-queries
title: "`getBy` for presence, `queryBy` for absence"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: auto
stability: preview
ported_from: ["eslint-plugin-testing-library/prefer-presence-queries"]
since: 0.5.0
---

An assertion that an element is present uses `getBy…` (`expect(screen.getByText("x")).toBeInTheDocument()`), and one that it is absent uses `queryBy…` (`.not.toBeInTheDocument()`). The fix swaps the query.

**Why:** `getBy` throws with a helpful message when the element is missing; `queryBy` returns `null`, which is what an absence check needs.

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
    - all: [{ pattern: "expect(screen.queryByRole($$$A)).toBeInTheDocument()" }, { pattern: $W0 }]
    - all: [{ pattern: "expect(screen.queryByLabelText($$$A)).toBeInTheDocument()" }, { pattern: $W1 }]
    - all: [{ pattern: "expect(screen.queryByPlaceholderText($$$A)).toBeInTheDocument()" }, { pattern: $W2 }]
    - all: [{ pattern: "expect(screen.queryByText($$$A)).toBeInTheDocument()" }, { pattern: $W3 }]
    - all: [{ pattern: "expect(screen.queryByDisplayValue($$$A)).toBeInTheDocument()" }, { pattern: $W4 }]
    - all: [{ pattern: "expect(screen.queryByAltText($$$A)).toBeInTheDocument()" }, { pattern: $W5 }]
    - all: [{ pattern: "expect(screen.queryByTitle($$$A)).toBeInTheDocument()" }, { pattern: $W6 }]
    - all: [{ pattern: "expect(screen.queryByTestId($$$A)).toBeInTheDocument()" }, { pattern: $W7 }]
    - all: [{ pattern: "expect(screen.queryAllByRole($$$A)).toBeInTheDocument()" }, { pattern: $W8 }]
    - all: [{ pattern: "expect(screen.queryAllByLabelText($$$A)).toBeInTheDocument()" }, { pattern: $W9 }]
    - all: [{ pattern: "expect(screen.queryAllByPlaceholderText($$$A)).toBeInTheDocument()" }, { pattern: $W10 }]
    - all: [{ pattern: "expect(screen.queryAllByText($$$A)).toBeInTheDocument()" }, { pattern: $W11 }]
    - all: [{ pattern: "expect(screen.queryAllByDisplayValue($$$A)).toBeInTheDocument()" }, { pattern: $W12 }]
    - all: [{ pattern: "expect(screen.queryAllByAltText($$$A)).toBeInTheDocument()" }, { pattern: $W13 }]
    - all: [{ pattern: "expect(screen.queryAllByTitle($$$A)).toBeInTheDocument()" }, { pattern: $W14 }]
    - all: [{ pattern: "expect(screen.queryAllByTestId($$$A)).toBeInTheDocument()" }, { pattern: $W15 }]
    - all: [{ pattern: "expect(screen.getByRole($$$A)).not.toBeInTheDocument()" }, { pattern: $W16 }]
    - all: [{ pattern: "expect(screen.getByLabelText($$$A)).not.toBeInTheDocument()" }, { pattern: $W17 }]
    - all: [{ pattern: "expect(screen.getByPlaceholderText($$$A)).not.toBeInTheDocument()" }, { pattern: $W18 }]
    - all: [{ pattern: "expect(screen.getByText($$$A)).not.toBeInTheDocument()" }, { pattern: $W19 }]
    - all: [{ pattern: "expect(screen.getByDisplayValue($$$A)).not.toBeInTheDocument()" }, { pattern: $W20 }]
    - all: [{ pattern: "expect(screen.getByAltText($$$A)).not.toBeInTheDocument()" }, { pattern: $W21 }]
    - all: [{ pattern: "expect(screen.getByTitle($$$A)).not.toBeInTheDocument()" }, { pattern: $W22 }]
    - all: [{ pattern: "expect(screen.getByTestId($$$A)).not.toBeInTheDocument()" }, { pattern: $W23 }]
    - all: [{ pattern: "expect(screen.getAllByRole($$$A)).not.toBeInTheDocument()" }, { pattern: $W24 }]
    - all: [{ pattern: "expect(screen.getAllByLabelText($$$A)).not.toBeInTheDocument()" }, { pattern: $W25 }]
    - all: [{ pattern: "expect(screen.getAllByPlaceholderText($$$A)).not.toBeInTheDocument()" }, { pattern: $W26 }]
    - all: [{ pattern: "expect(screen.getAllByText($$$A)).not.toBeInTheDocument()" }, { pattern: $W27 }]
    - all: [{ pattern: "expect(screen.getAllByDisplayValue($$$A)).not.toBeInTheDocument()" }, { pattern: $W28 }]
    - all: [{ pattern: "expect(screen.getAllByAltText($$$A)).not.toBeInTheDocument()" }, { pattern: $W29 }]
    - all: [{ pattern: "expect(screen.getAllByTitle($$$A)).not.toBeInTheDocument()" }, { pattern: $W30 }]
    - all: [{ pattern: "expect(screen.getAllByTestId($$$A)).not.toBeInTheDocument()" }, { pattern: $W31 }]
message: "Use `getBy` to assert presence and `queryBy` to assert absence"
fix:
  - replace: { target: $W0, with: 'expect(screen.getByRole($$$A)).toBeInTheDocument()' }
  - replace: { target: $W1, with: 'expect(screen.getByLabelText($$$A)).toBeInTheDocument()' }
  - replace: { target: $W2, with: 'expect(screen.getByPlaceholderText($$$A)).toBeInTheDocument()' }
  - replace: { target: $W3, with: 'expect(screen.getByText($$$A)).toBeInTheDocument()' }
  - replace: { target: $W4, with: 'expect(screen.getByDisplayValue($$$A)).toBeInTheDocument()' }
  - replace: { target: $W5, with: 'expect(screen.getByAltText($$$A)).toBeInTheDocument()' }
  - replace: { target: $W6, with: 'expect(screen.getByTitle($$$A)).toBeInTheDocument()' }
  - replace: { target: $W7, with: 'expect(screen.getByTestId($$$A)).toBeInTheDocument()' }
  - replace: { target: $W8, with: 'expect(screen.getAllByRole($$$A)).toBeInTheDocument()' }
  - replace: { target: $W9, with: 'expect(screen.getAllByLabelText($$$A)).toBeInTheDocument()' }
  - replace: { target: $W10, with: 'expect(screen.getAllByPlaceholderText($$$A)).toBeInTheDocument()' }
  - replace: { target: $W11, with: 'expect(screen.getAllByText($$$A)).toBeInTheDocument()' }
  - replace: { target: $W12, with: 'expect(screen.getAllByDisplayValue($$$A)).toBeInTheDocument()' }
  - replace: { target: $W13, with: 'expect(screen.getAllByAltText($$$A)).toBeInTheDocument()' }
  - replace: { target: $W14, with: 'expect(screen.getAllByTitle($$$A)).toBeInTheDocument()' }
  - replace: { target: $W15, with: 'expect(screen.getAllByTestId($$$A)).toBeInTheDocument()' }
  - replace: { target: $W16, with: 'expect(screen.queryByRole($$$A)).not.toBeInTheDocument()' }
  - replace: { target: $W17, with: 'expect(screen.queryByLabelText($$$A)).not.toBeInTheDocument()' }
  - replace: { target: $W18, with: 'expect(screen.queryByPlaceholderText($$$A)).not.toBeInTheDocument()' }
  - replace: { target: $W19, with: 'expect(screen.queryByText($$$A)).not.toBeInTheDocument()' }
  - replace: { target: $W20, with: 'expect(screen.queryByDisplayValue($$$A)).not.toBeInTheDocument()' }
  - replace: { target: $W21, with: 'expect(screen.queryByAltText($$$A)).not.toBeInTheDocument()' }
  - replace: { target: $W22, with: 'expect(screen.queryByTitle($$$A)).not.toBeInTheDocument()' }
  - replace: { target: $W23, with: 'expect(screen.queryByTestId($$$A)).not.toBeInTheDocument()' }
  - replace: { target: $W24, with: 'expect(screen.queryAllByRole($$$A)).not.toBeInTheDocument()' }
  - replace: { target: $W25, with: 'expect(screen.queryAllByLabelText($$$A)).not.toBeInTheDocument()' }
  - replace: { target: $W26, with: 'expect(screen.queryAllByPlaceholderText($$$A)).not.toBeInTheDocument()' }
  - replace: { target: $W27, with: 'expect(screen.queryAllByText($$$A)).not.toBeInTheDocument()' }
  - replace: { target: $W28, with: 'expect(screen.queryAllByDisplayValue($$$A)).not.toBeInTheDocument()' }
  - replace: { target: $W29, with: 'expect(screen.queryAllByAltText($$$A)).not.toBeInTheDocument()' }
  - replace: { target: $W30, with: 'expect(screen.queryAllByTitle($$$A)).not.toBeInTheDocument()' }
  - replace: { target: $W31, with: 'expect(screen.queryAllByTestId($$$A)).not.toBeInTheDocument()' }
```

## Examples

### JavaScript

#### Don't

```js
test("t", () => {
  expect(screen.queryByText("Hi")).toBeInTheDocument();
});
```

#### Do

```js
test("t", () => {
  expect(screen.getByText("Hi")).toBeInTheDocument();
});
```

### JSX

#### Don't

```jsx
test("t", () => {
  expect(screen.queryByText("Hi")).toBeInTheDocument();
});
```

#### Do

```jsx
test("t", () => {
  expect(screen.getByText("Hi")).toBeInTheDocument();
});
```

### TypeScript

#### Don't

```ts
test("t", () => {
  expect(screen.queryByText("Hi")).toBeInTheDocument();
});
```

#### Do

```ts
test("t", () => {
  expect(screen.getByText("Hi")).toBeInTheDocument();
});
```

### TSX

#### Don't

```tsx
test("t", () => {
  expect(screen.queryByText("Hi")).toBeInTheDocument();
});
```

#### Do

```tsx
test("t", () => {
  expect(screen.getByText("Hi")).toBeInTheDocument();
});
```
