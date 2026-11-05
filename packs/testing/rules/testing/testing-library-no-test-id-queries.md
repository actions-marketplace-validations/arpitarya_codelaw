---
id: testing/testing-library-no-test-id-queries
title: "No queries by test id"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: manual
fix_reason: "the query becomes one by role, label or text, which needs knowing what the user sees"
fix_exception: restructure
stability: preview
ported_from: ["eslint-plugin-testing-library/no-test-id-queries"]
since: 0.5.0
---

A test does not query by `data-testid` (`getByTestId` and its kin). Opt-in.

**Why:** Users cannot see test ids; a query by role, label or text tests what they can, and fails when that breaks.

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
  has: { field: function, regex: '(^|\.)(get|getAll|query|queryAll|find|findAll)ByTestId$' }
message: "Query by role, label or text instead of a test id"
```

## How to fix

1. Replace it with `getByRole`, `getByLabelText` or `getByText` for what the user sees.

## Examples

### JavaScript

#### Don't

```js
test("t", () => {
  screen.getByTestId("save");
});
```

#### Do

```js
test("t", () => {
  screen.getByRole("button", { name: "Save" });
});
```

### JSX

#### Don't

```jsx
test("t", () => {
  screen.getByTestId("save");
});
```

#### Do

```jsx
test("t", () => {
  screen.getByRole("button", { name: "Save" });
});
```

### TypeScript

#### Don't

```ts
test("t", () => {
  screen.getByTestId("save");
});
```

#### Do

```ts
test("t", () => {
  screen.getByRole("button", { name: "Save" });
});
```

### TSX

#### Don't

```tsx
test("t", () => {
  screen.getByTestId("save");
});
```

#### Do

```tsx
test("t", () => {
  screen.getByRole("button", { name: "Save" });
});
```
