---
id: testing/no-conditional-tests
title: "No tests declared inside a condition"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: manual
fix_reason: "the condition becomes `test.skipIf`/`runIf` or a test of its own, which the author chooses"
fix_exception: restructure
stability: preview
ported_from: ["@vitest/eslint-plugin/no-conditional-tests"]
since: 0.5.0
---

A test or `describe` is not declared inside an `if`, a `switch` or a `? :`; use `test.skipIf(…)`/`test.runIf(…)` instead.

**Why:** A conditionally declared test disappears from the report when the condition is false, so nobody sees that it was skipped.

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
  any: [{ matches: test-call }, { matches: describe-call }]
  inside: { stopBy: end, any: [{ kind: if_statement }, { kind: switch_statement }, { kind: ternary_expression }] }
message: "Use `test.skipIf` or `test.runIf` instead of a condition"
```

## How to fix

1. Replace `if (cond) { test(…) }` with `test.runIf(cond)(…)`, or `test.skipIf(!cond)(…)`.

## Examples

### JavaScript

#### Don't

```js
if (process.env.CI) {
  test("slow", () => {});
}
```

#### Do

```js
test.runIf(process.env.CI)("slow", () => {});
```

### JSX

#### Don't

```jsx
if (process.env.CI) {
  test("slow", () => {});
}
```

#### Do

```jsx
test.runIf(process.env.CI)("slow", () => {});
```

### TypeScript

#### Don't

```ts
if (process.env.CI) {
  test("slow", () => {});
}
```

#### Do

```ts
test.runIf(process.env.CI)("slow", () => {});
```

### TSX

#### Don't

```tsx
if (process.env.CI) {
  test("slow", () => {});
}
```

#### Do

```tsx
test.runIf(process.env.CI)("slow", () => {});
```
