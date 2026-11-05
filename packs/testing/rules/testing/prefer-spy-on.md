---
id: testing/prefer-spy-on
title: "`spyOn` instead of overwriting a method"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: auto
stability: preview
ported_from: ["eslint-plugin-jest/prefer-spy-on", "@vitest/eslint-plugin/prefer-spy-on"]
since: 0.5.0
---

A method is mocked with `jest.spyOn(obj, "m").mockImplementation(…)` (or `vi.spyOn`), not by assigning `obj.m = jest.fn(…)`.

**Why:** A spy can be restored with `mockRestore()` or `restoreAllMocks`; an assigned mock replaces the method for every later test in the file.

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
  kind: assignment_expression
  any:
    - all: [{ pattern: "$O.$P = jest.fn()" }, { pattern: $W0 }]
    - all: [{ pattern: "$O.$P = jest.fn($F)" }, { pattern: $W1 }]
    - all: [{ pattern: "$O.$P = vi.fn()" }, { pattern: $W2 }]
    - all: [{ pattern: "$O.$P = vi.fn($F)" }, { pattern: $W3 }]
constraints:
  P: { kind: property_identifier }
message: "Use `spyOn` to mock a method"
fix:
  - replace: { target: $W0, with: 'jest.spyOn($O, "$P").mockImplementation()' }
  - replace: { target: $W1, with: 'jest.spyOn($O, "$P").mockImplementation($F)' }
  - replace: { target: $W2, with: 'vi.spyOn($O, "$P").mockImplementation(() => undefined)' }
  - replace: { target: $W3, with: 'vi.spyOn($O, "$P").mockImplementation($F)' }
```

## Examples

### JavaScript

#### Don't

```js
test("logs", () => {
  console.error = jest.fn();
});
```

#### Do

```js
test("logs", () => {
  jest.spyOn(console, "error").mockImplementation();
});
```

### JSX

#### Don't

```jsx
test("logs", () => {
  console.error = jest.fn();
});
```

#### Do

```jsx
test("logs", () => {
  jest.spyOn(console, "error").mockImplementation();
});
```

### TypeScript

#### Don't

```ts
test("logs", () => {
  console.error = jest.fn();
});
```

#### Do

```ts
test("logs", () => {
  jest.spyOn(console, "error").mockImplementation();
});
```

### TSX

#### Don't

```tsx
test("logs", () => {
  console.error = jest.fn();
});
```

#### Do

```tsx
test("logs", () => {
  jest.spyOn(console, "error").mockImplementation();
});
```
