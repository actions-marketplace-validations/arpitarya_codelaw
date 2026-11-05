---
id: testing/prefer-mock-return-shorthand
title: "`mockReturnValue` for a constant"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: auto
stability: preview
ported_from: ["eslint-plugin-jest/prefer-mock-return-shorthand", "@vitest/eslint-plugin/prefer-mock-return-shorthand"]
since: 0.5.0
---

A mock implementation that only returns a primitive literal, `mockImplementation(() => 42)`, is written `mockReturnValue(42)` (and the `Once` form). An implementation that builds an object, reads a variable or calls something is left alone: it runs on every call, and the shorthand would evaluate it once.

**Why:** The shorthand states that the mock returns a constant.

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
    - all: [{ pattern: '$O.mockImplementation(() => $V)' }, { pattern: $W0 }]
    - all: [{ pattern: '$O.mockImplementationOnce(() => $V)' }, { pattern: $W1 }]
constraints:
  V: { any: [{ kind: number }, { kind: string }, { kind: 'true' }, { kind: 'false' }, { kind: 'null' }, { kind: undefined }] }
message: "Use `mockReturnValue` for a mock that returns a constant"
fix:
  - replace: { target: $W0, with: '$O.mockReturnValue($V)' }
  - replace: { target: $W1, with: '$O.mockReturnValueOnce($V)' }
```

## Examples

### JavaScript

#### Don't

```js
test("counts", () => {
  count.mockImplementation(() => 3);
});
```

#### Do

```js
test("counts", () => {
  count.mockReturnValue(3);
});
```

### JSX

#### Don't

```jsx
test("counts", () => {
  count.mockImplementation(() => 3);
});
```

#### Do

```jsx
test("counts", () => {
  count.mockReturnValue(3);
});
```

### TypeScript

#### Don't

```ts
test("counts", () => {
  count.mockImplementation(() => 3);
});
```

#### Do

```ts
test("counts", () => {
  count.mockReturnValue(3);
});
```

### TSX

#### Don't

```tsx
test("counts", () => {
  count.mockImplementation(() => 3);
});
```

#### Do

```tsx
test("counts", () => {
  count.mockReturnValue(3);
});
```
