---
id: testing/no-restricted-matchers
title: "No matchers the project restricts"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: manual
fix_reason: "what to use instead is the project's convention"
fix_exception: value
stability: preview
options: { matchers: { type: "string[]", default: [] } }
ported_from: ["eslint-plugin-jest/no-restricted-matchers", "@vitest/eslint-plugin/no-restricted-matchers"]
since: 0.5.0
---

A matcher the project lists by name in `matchers` is reported: `matchers = ["toBeFalsy", "toBeTruthy"]`. The list is empty by default. (The source also takes modifier chains such as `not.toHaveBeenCalledWith`; here the name is the matcher's own.)

**Why:** A project bans a matcher when a stricter one says more (`toBe(false)` over `toBeFalsy()`).

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
  has: { field: function, has: { field: property, pattern: $M } }
where:
  $M: { in: matchers }
message: "`$M` is restricted in this project"
```

## How to fix

1. Use the matcher the project prefers.

## Examples

### JavaScript

#### Don't

```js options={"matchers":["toBeFalsy"]}
test("t", () => {
  expect(ok).toBeFalsy();
});
```

#### Do

```js options={"matchers":["toBeFalsy"]}
test("t", () => {
  expect(ok).toBe(false);
});
```

### JSX

#### Don't

```jsx options={"matchers":["toBeFalsy"]}
test("t", () => {
  expect(ok).toBeFalsy();
});
```

#### Do

```jsx options={"matchers":["toBeFalsy"]}
test("t", () => {
  expect(ok).toBe(false);
});
```

### TypeScript

#### Don't

```ts options={"matchers":["toBeFalsy"]}
test("t", () => {
  expect(ok).toBeFalsy();
});
```

#### Do

```ts options={"matchers":["toBeFalsy"]}
test("t", () => {
  expect(ok).toBe(false);
});
```

### TSX

#### Don't

```tsx options={"matchers":["toBeFalsy"]}
test("t", () => {
  expect(ok).toBeFalsy();
});
```

#### Do

```tsx options={"matchers":["toBeFalsy"]}
test("t", () => {
  expect(ok).toBe(false);
});
```
