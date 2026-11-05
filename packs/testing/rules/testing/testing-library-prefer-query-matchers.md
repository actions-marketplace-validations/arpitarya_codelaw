---
id: testing/testing-library-prefer-query-matchers
title: "The query each matcher is meant for"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: manual
fix_reason: "which query pairs with which matcher is the project's configuration"
fix_exception: value
stability: preview
options: { get_matchers: { type: "string[]", default: [] }, query_matchers: { type: "string[]", default: [] } }
ported_from: ["eslint-plugin-testing-library/prefer-query-matchers"]
since: 0.5.0
---

A matcher the project lists in `get_matchers` is used with a `getBy…` query, and one it lists in `query_matchers` with a `queryBy…` query: `get_matchers = ["toBeVisible"]` reports `expect(screen.queryByText("x")).toBeVisible()`. Both lists are empty by default.

**Why:** `getBy` fails with the DOM printed when the element is missing; `queryBy` returns `null` for absence checks. Pairing each matcher with its query keeps failures readable.

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
  matches: matcher-call
  any:
    - all:
      - has: { field: function, has: { field: property, pattern: $G } }
      - has: { field: function, has: { field: object, stopBy: end, kind: call_expression, has: { field: function, regex: '(^|\.)queryBy(Role|LabelText|PlaceholderText|Text|DisplayValue|AltText|Title|TestId)$' } } }
    - all:
      - has: { field: function, has: { field: property, pattern: $P } }
      - has: { field: function, has: { field: object, stopBy: end, kind: call_expression, has: { field: function, regex: '(^|\.)getBy(Role|LabelText|PlaceholderText|Text|DisplayValue|AltText|Title|TestId)$' } } }
where:
  $G: { in: get_matchers }
  $P: { in: query_matchers }
message: "Use the query this matcher is meant for"
```

## How to fix

1. Swap `queryBy…` for `getBy…` (or the reverse) as the project's pairing says.

## Examples

### JavaScript

#### Don't

```js options={"get_matchers":["toBeVisible"]}
test("t", () => {
  expect(screen.queryByText("Hi")).toBeVisible();
});
```

#### Do

```js options={"get_matchers":["toBeVisible"]}
test("t", () => {
  expect(screen.getByText("Hi")).toBeVisible();
});
```

### JSX

#### Don't

```jsx options={"get_matchers":["toBeVisible"]}
test("t", () => {
  expect(screen.queryByText("Hi")).toBeVisible();
});
```

#### Do

```jsx options={"get_matchers":["toBeVisible"]}
test("t", () => {
  expect(screen.getByText("Hi")).toBeVisible();
});
```

### TypeScript

#### Don't

```ts options={"get_matchers":["toBeVisible"]}
test("t", () => {
  expect(screen.queryByText("Hi")).toBeVisible();
});
```

#### Do

```ts options={"get_matchers":["toBeVisible"]}
test("t", () => {
  expect(screen.getByText("Hi")).toBeVisible();
});
```

### TSX

#### Don't

```tsx options={"get_matchers":["toBeVisible"]}
test("t", () => {
  expect(screen.queryByText("Hi")).toBeVisible();
});
```

#### Do

```tsx options={"get_matchers":["toBeVisible"]}
test("t", () => {
  expect(screen.getByText("Hi")).toBeVisible();
});
```
