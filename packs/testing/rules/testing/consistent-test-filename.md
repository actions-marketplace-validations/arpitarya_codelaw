---
id: testing/consistent-test-filename
title: "Test files named `*.test.*`"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.spec.*"]
fix: manual
fix_reason: "renaming the file changes every path that names it"
fix_exception: restructure
stability: preview
ported_from: ["@vitest/eslint-plugin/consistent-test-filename"]
since: 0.5.0
---

A file holding tests is named `<name>.test.<ext>`, not `<name>.spec.<ext>` (the source's default pattern). Opt-in.

**Why:** One naming pattern lets the runner's `include`, the editor and CI find every test file the same way.

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
  kind: program
  has: { stopBy: end, matches: test-call }
message: "Name this test file `*.test.*`"
```

## How to fix

1. Rename the file from `<name>.spec.<ext>` to `<name>.test.<ext>`.

## Examples

### JavaScript

#### Don't

```js path=src/cart.spec.js
test("adds", () => {
  expect(1 + 1).toBe(2);
});
```

#### Do

```js path=src/cart.spec.js
export const add = (a, b) => a + b;
```

### JSX

#### Don't

```jsx path=src/cart.spec.jsx
test("adds", () => {
  expect(1 + 1).toBe(2);
});
```

#### Do

```jsx path=src/cart.spec.jsx
export const add = (a, b) => a + b;
```

### TypeScript

#### Don't

```ts path=src/cart.spec.ts
test("adds", () => {
  expect(1 + 1).toBe(2);
});
```

#### Do

```ts path=src/cart.spec.ts
export const add = (a, b) => a + b;
```

### TSX

#### Don't

```tsx path=src/cart.spec.tsx
test("adds", () => {
  expect(1 + 1).toBe(2);
});
```

#### Do

```tsx path=src/cart.spec.tsx
export const add = (a, b) => a + b;
```
