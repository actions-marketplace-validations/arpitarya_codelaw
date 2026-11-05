---
id: testing/require-to-throw-message
title: "`toThrow` names the error"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: manual
fix_reason: "the expected message or error class is the author's to write"
fix_exception: value
stability: preview
ported_from: ["eslint-plugin-jest/require-to-throw-message", "@vitest/eslint-plugin/require-to-throw-message"]
since: 0.5.0
---

`toThrow()` and `toThrowError()` are given the expected message, pattern or error class. Negated forms are left alone.

**Why:** An empty `toThrow()` passes on any error, a `TypeError` from a typo included.

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
  has: { field: arguments, not: { has: { nthChild: 1 } } }
  all:
    - has: { field: function, has: { field: property, regex: '^(toThrow|toThrowError)$' } }
    - has: { field: function, not: { regex: '\.not\.' } }
message: "Say which error is expected"
```

## How to fix

1. Pass the message, a regular expression or the error class: `toThrow("not found")`, `toThrow(NotFoundError)`.

## Examples

### JavaScript

#### Don't

```js
test("rejects", () => {
  expect(() => parse("")).toThrow();
});
```

#### Do

```js
test("rejects", () => {
  expect(() => parse("")).toThrow("empty input");
});
```

### JSX

#### Don't

```jsx
test("rejects", () => {
  expect(() => parse("")).toThrow();
});
```

#### Do

```jsx
test("rejects", () => {
  expect(() => parse("")).toThrow("empty input");
});
```

### TypeScript

#### Don't

```ts
test("rejects", () => {
  expect(() => parse("")).toThrow();
});
```

#### Do

```ts
test("rejects", () => {
  expect(() => parse("")).toThrow("empty input");
});
```

### TSX

#### Don't

```tsx
test("rejects", () => {
  expect(() => parse("")).toThrow();
});
```

#### Do

```tsx
test("rejects", () => {
  expect(() => parse("")).toThrow("empty input");
});
```
