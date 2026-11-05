---
id: testing/no-confusing-set-timeout
title: "`jest.setTimeout` once, at the top of the file"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: suggest
fix_reason: "moving the call changes the timeout of the tests above it"
stability: preview
ported_from: ["eslint-plugin-jest/no-confusing-set-timeout"]
since: 0.5.0
---

`jest.setTimeout` is called at the top level of a test file, not inside a `describe`, a test or a hook. The suggested edit moves it after the imports.

**Why:** It sets the timeout for the whole file whenever it runs, so a call inside a block reads as if it were scoped to that block, and it is not.

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
  kind: expression_statement
  has: { kind: call_expression, has: { field: function, regex: '^jest\.setTimeout$' } }
  inside: { stopBy: end, any: [{ kind: arrow_function }, { kind: function_expression }, { kind: function_declaration }] }
message: "`jest.setTimeout` applies to the whole file; call it at the top level"
fix:
  - move-to: after-imports
```

## Examples

### JavaScript

#### Don't

```js
describe("slow", () => {
  jest.setTimeout(10000);
  test("loads", () => {});
});
```

#### Do

```js
jest.setTimeout(10000);
describe("slow", () => {
  test("loads", () => {});
});
```

### JSX

#### Don't

```jsx
describe("slow", () => {
  jest.setTimeout(10000);
  test("loads", () => {});
});
```

#### Do

```jsx
jest.setTimeout(10000);
describe("slow", () => {
  test("loads", () => {});
});
```

### TypeScript

#### Don't

```ts
describe("slow", () => {
  jest.setTimeout(10000);
  test("loads", () => {});
});
```

#### Do

```ts
jest.setTimeout(10000);
describe("slow", () => {
  test("loads", () => {});
});
```

### TSX

#### Don't

```tsx
describe("slow", () => {
  jest.setTimeout(10000);
  test("loads", () => {});
});
```

#### Do

```tsx
jest.setTimeout(10000);
describe("slow", () => {
  test("loads", () => {});
});
```
