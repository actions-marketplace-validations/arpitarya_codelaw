---
id: testing/testing-library-prefer-explicit-assert
title: "An explicit assertion on a `getBy` query"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: suggest
fix_reason: "the matcher the author meant may be another (`toBeVisible`, `toHaveTextContent`)"
stability: preview
ported_from: ["eslint-plugin-testing-library/prefer-explicit-assert"]
since: 0.5.0
---

A `getBy…` query used only to check that an element exists is wrapped in an assertion: `expect(screen.getByText("Hi")).toBeInTheDocument()`. The suggested edit adds it. Opt-in; the opposite of `prefer-implicit-assert`.

**Why:** A bare query asserts by throwing, which reads as setup; an `expect` says that the presence is what is tested.

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
  has: { nthChild: 1, pattern: $C, kind: call_expression, has: { field: function, regex: '(^|\.)getBy(Role|LabelText|PlaceholderText|Text|DisplayValue|AltText|Title|TestId)$' } }
message: "Assert the query explicitly"
fix: 'expect($C).toBeInTheDocument();'
```

## Examples

### JavaScript

#### Don't

```js
test("t", () => {
  screen.getByText("Hi");
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
  screen.getByText("Hi");
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
  screen.getByText("Hi");
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
  screen.getByText("Hi");
});
```

#### Do

```tsx
test("t", () => {
  expect(screen.getByText("Hi")).toBeInTheDocument();
});
```
