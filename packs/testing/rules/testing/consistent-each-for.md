---
id: testing/consistent-each-for
title: "`.for` over `.each` in Vitest"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: manual
fix_reason: "`.for` passes each row as one argument, so the callback's parameters change shape"
fix_exception: restructure
stability: preview
ported_from: ["@vitest/eslint-plugin/consistent-each-for"]
since: 0.5.0
---

A parameterised Vitest test uses `test.for` / `describe.for`, not `.each`. Opt-in: it suits a project that has chosen `.for`.

**Why:** `.for` passes the row as one value and gives the callback the test context as its second argument, which `.each` cannot; one form per project keeps the tables readable.

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
  has:
    field: function
    kind: member_expression
    all:
      - has: { field: property, regex: '^each$' }
      - regex: '^(it|test|describe|suite)(\.(only|skip|concurrent))*\.each$'
message: "Use `.for` instead of `.each`"
```

## How to fix

1. Replace `.each` with `.for`.
2. Destructure the row in the callback's first parameter: `([a, b]) => …`.

## Examples

### JavaScript

#### Don't

```js
test.each([[1, 2]])("adds %i", (a, b) => {
  expect(a + 1).toBe(b);
});
```

#### Do

```js
test.for([[1, 2]])("adds %i", ([a, b]) => {
  expect(a + 1).toBe(b);
});
```

### JSX

#### Don't

```jsx
test.each([[1, 2]])("adds %i", (a, b) => {
  expect(a + 1).toBe(b);
});
```

#### Do

```jsx
test.for([[1, 2]])("adds %i", ([a, b]) => {
  expect(a + 1).toBe(b);
});
```

### TypeScript

#### Don't

```ts
test.each([[1, 2]])("adds %i", (a, b) => {
  expect(a + 1).toBe(b);
});
```

#### Do

```ts
test.for([[1, 2]])("adds %i", ([a, b]) => {
  expect(a + 1).toBe(b);
});
```

### TSX

#### Don't

```tsx
test.each([[1, 2]])("adds %i", (a, b) => {
  expect(a + 1).toBe(b);
});
```

#### Do

```tsx
test.for([[1, 2]])("adds %i", ([a, b]) => {
  expect(a + 1).toBe(b);
});
```
