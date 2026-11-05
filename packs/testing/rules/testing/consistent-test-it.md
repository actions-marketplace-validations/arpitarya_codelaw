---
id: testing/consistent-test-it
title: "`test` at the top level, `it` inside `describe`"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: auto
stability: preview
ported_from: ["eslint-plugin-jest/consistent-test-it", "@vitest/eslint-plugin/consistent-test-it"]
since: 0.5.0
---

A test at the top level of a file is declared with `test`, and one inside a `describe` with `it`, so a suite reads `describe("x", () => { it("does y") })` and a flat file `test("x does y")`. (The source's defaults; the two names are aliases.)

**Why:** Mixing the two aliases in one file makes it harder to read and search.

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
  kind: identifier
  all:
    - any:
      - inside: { kind: call_expression, field: function }
      - inside: { kind: member_expression, field: object, inside: { kind: call_expression, field: function } }
    - any:
      - { regex: '^it$', pattern: $IT, not: { inside: { stopBy: end, matches: describe-call } } }
      - { regex: '^test$', pattern: $TEST, inside: { stopBy: end, matches: describe-call } }
message: "Use `test` at the top level and `it` inside `describe`"
fix:
  - replace: { target: $IT, with: test }
  - replace: { target: $TEST, with: it }
```

## Examples

### JavaScript

#### Don't

```js
it("adds", () => {
  expect(add(1, 2)).toBe(3);
});

describe("sub", () => {
  test("subtracts", () => {});
});
```

#### Do

```js
test("adds", () => {
  expect(add(1, 2)).toBe(3);
});

describe("sub", () => {
  it("subtracts", () => {});
});
```

### JSX

#### Don't

```jsx
it("adds", () => {
  expect(add(1, 2)).toBe(3);
});

describe("sub", () => {
  test("subtracts", () => {});
});
```

#### Do

```jsx
test("adds", () => {
  expect(add(1, 2)).toBe(3);
});

describe("sub", () => {
  it("subtracts", () => {});
});
```

### TypeScript

#### Don't

```ts
it("adds", () => {
  expect(add(1, 2)).toBe(3);
});

describe("sub", () => {
  test("subtracts", () => {});
});
```

#### Do

```ts
test("adds", () => {
  expect(add(1, 2)).toBe(3);
});

describe("sub", () => {
  it("subtracts", () => {});
});
```

### TSX

#### Don't

```tsx
it("adds", () => {
  expect(add(1, 2)).toBe(3);
});

describe("sub", () => {
  test("subtracts", () => {});
});
```

#### Do

```tsx
test("adds", () => {
  expect(add(1, 2)).toBe(3);
});

describe("sub", () => {
  it("subtracts", () => {});
});
```
