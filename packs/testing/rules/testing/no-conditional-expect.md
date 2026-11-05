---
id: testing/no-conditional-expect
title: "No `expect` inside a condition"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: manual
fix_reason: "the test has to be split or its setup fixed so the assertion always runs; no edit can tell which"
fix_exception: restructure
stability: preview
ported_from: ["eslint-plugin-jest/no-conditional-expect", "@vitest/eslint-plugin/no-conditional-expect"]
since: 0.5.0
---

A test does not call `expect` inside an `if`, a `switch`, a `catch`, a `? :` or a `&&`/`||`, where it may silently not run.

**Why:** A conditional assertion passes when the condition is false, so the test can pass while checking nothing.

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
  matches: expect-call
  inside:
    stopBy: end
    any: [{ kind: if_statement }, { kind: switch_statement }, { kind: catch_clause }, { kind: ternary_expression }, { kind: binary_expression, regex: '&&|\|\||\?\?' }]
    inside: { stopBy: end, matches: test-fn }
message: "This `expect` may not run: it is inside a condition"
```

## How to fix

1. Split the test so each case sets up the state it asserts on, and assert unconditionally.
2. For an expected error, use `expect(() => …).toThrow()` or `await expect(p).rejects…` instead of `try`/`catch`.

## Examples

### JavaScript

#### Don't

```js
test("parses", () => {
  if (input) {
    expect(parse(input)).toBe(1);
  }
});
```

#### Do

```js
test("parses", () => {
  expect(parse("1")).toBe(1);
});
```

### JSX

#### Don't

```jsx
test("parses", () => {
  if (input) {
    expect(parse(input)).toBe(1);
  }
});
```

#### Do

```jsx
test("parses", () => {
  expect(parse("1")).toBe(1);
});
```

### TypeScript

#### Don't

```ts
test("parses", () => {
  if (input) {
    expect(parse(input)).toBe(1);
  }
});
```

#### Do

```ts
test("parses", () => {
  expect(parse("1")).toBe(1);
});
```

### TSX

#### Don't

```tsx
test("parses", () => {
  if (input) {
    expect(parse(input)).toBe(1);
  }
});
```

#### Do

```tsx
test("parses", () => {
  expect(parse("1")).toBe(1);
});
```
