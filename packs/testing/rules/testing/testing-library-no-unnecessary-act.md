---
id: testing/testing-library-no-unnecessary-act
title: "No `act` around Testing Library calls"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: manual
fix_reason: "unwrapping the `act` callback moves its statements out, and an `async` one changes shape"
fix_exception: restructure
stability: preview
ported_from: ["eslint-plugin-testing-library/no-unnecessary-act"]
since: 0.5.0
---

`act(…)` does not wrap only Testing Library calls (`render`, `fireEvent`, `userEvent`, queries), which already run in `act`, and is not empty.

**Why:** The extra `act` adds noise and can hide the warning that points at a real missing one.

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
  all:
    - has: { field: function, regex: '^act$' }
    - has:
        field: arguments
        has:
          nthChild: 1
          all:
            - any: [{ kind: arrow_function }, { kind: function_expression }]
            - any:
              - has: { field: body, kind: statement_block, not: { has: { nthChild: 1 } } }
              - has: { field: body, kind: statement_block, all: [{ has: { kind: expression_statement } }, { not: { has: { any: [{ kind: lexical_declaration }, { kind: variable_declaration }, { kind: if_statement }, { kind: return_statement }, { kind: try_statement }, { kind: for_statement }, { kind: for_in_statement }] } } }, { not: { has: { kind: expression_statement, not: { has: { stopBy: end, kind: call_expression, has: { field: function, regex: '^(render|fireEvent|userEvent|screen)\b' } } } } } }] }
              - has: { field: body, kind: call_expression, has: { field: function, regex: '^(render|fireEvent|userEvent|screen)\b' } }
message: "This `act` is not needed"
```

## How to fix

1. Move the statements out of the `act` callback and delete the `act` call.

## Examples

### JavaScript

#### Don't

```js
test("t", () => {
  act(() => {
    fireEvent.click(button);
  });
});
```

#### Do

```js
test("t", () => {
  fireEvent.click(button);
});
```

### JSX

#### Don't

```jsx
test("t", () => {
  act(() => {
    fireEvent.click(button);
  });
});
```

#### Do

```jsx
test("t", () => {
  fireEvent.click(button);
});
```

### TypeScript

#### Don't

```ts
test("t", () => {
  act(() => {
    fireEvent.click(button);
  });
});
```

#### Do

```ts
test("t", () => {
  fireEvent.click(button);
});
```

### TSX

#### Don't

```tsx
test("t", () => {
  act(() => {
    fireEvent.click(button);
  });
});
```

#### Do

```tsx
test("t", () => {
  fireEvent.click(button);
});
```
