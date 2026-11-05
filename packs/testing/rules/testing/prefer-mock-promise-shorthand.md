---
id: testing/prefer-mock-promise-shorthand
title: "`mockResolvedValue` and `mockRejectedValue`"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: auto
stability: preview
ported_from: ["eslint-plugin-jest/prefer-mock-promise-shorthand", "@vitest/eslint-plugin/prefer-mock-promise-shorthand"]
since: 0.5.0
---

A mock that returns a settled promise uses the shorthand: `mockResolvedValue(x)` for `mockImplementation(() => Promise.resolve(x))` or `mockReturnValue(Promise.resolve(x))`, `mockRejectedValue(e)` for the rejecting form, and the `Once` variants.

**Why:** The shorthand says what the mock does in one word, and builds a fresh promise for each call.

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
    - all: [{ pattern: "$O.mockImplementation(() => Promise.resolve($V))" }, { pattern: $W0 }]
    - all: [{ pattern: "$O.mockImplementation(() => Promise.reject($V))" }, { pattern: $W1 }]
    - all: [{ pattern: "$O.mockImplementationOnce(() => Promise.resolve($V))" }, { pattern: $W2 }]
    - all: [{ pattern: "$O.mockImplementationOnce(() => Promise.reject($V))" }, { pattern: $W3 }]
    - all: [{ pattern: "$O.mockReturnValue(Promise.resolve($V))" }, { pattern: $W4 }]
    - all: [{ pattern: "$O.mockReturnValue(Promise.reject($V))" }, { pattern: $W5 }]
    - all: [{ pattern: "$O.mockReturnValueOnce(Promise.resolve($V))" }, { pattern: $W6 }]
    - all: [{ pattern: "$O.mockReturnValueOnce(Promise.reject($V))" }, { pattern: $W7 }]
message: "Use the promise shorthand"
fix:
  - replace: { target: $W0, with: '$O.mockResolvedValue($V)' }
  - replace: { target: $W1, with: '$O.mockRejectedValue($V)' }
  - replace: { target: $W2, with: '$O.mockResolvedValueOnce($V)' }
  - replace: { target: $W3, with: '$O.mockRejectedValueOnce($V)' }
  - replace: { target: $W4, with: '$O.mockResolvedValue($V)' }
  - replace: { target: $W5, with: '$O.mockRejectedValue($V)' }
  - replace: { target: $W6, with: '$O.mockResolvedValueOnce($V)' }
  - replace: { target: $W7, with: '$O.mockRejectedValueOnce($V)' }
```

## Examples

### JavaScript

#### Don't

```js
test("loads", () => {
  api.get.mockImplementation(() => Promise.resolve(user));
});
```

#### Do

```js
test("loads", () => {
  api.get.mockResolvedValue(user);
});
```

### JSX

#### Don't

```jsx
test("loads", () => {
  api.get.mockImplementation(() => Promise.resolve(user));
});
```

#### Do

```jsx
test("loads", () => {
  api.get.mockResolvedValue(user);
});
```

### TypeScript

#### Don't

```ts
test("loads", () => {
  api.get.mockImplementation(() => Promise.resolve(user));
});
```

#### Do

```ts
test("loads", () => {
  api.get.mockResolvedValue(user);
});
```

### TSX

#### Don't

```tsx
test("loads", () => {
  api.get.mockImplementation(() => Promise.resolve(user));
});
```

#### Do

```tsx
test("loads", () => {
  api.get.mockResolvedValue(user);
});
```
