---
id: testing/playwright-no-identical-title
title: "No two tests with one title"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: manual
fix_reason: "a new title has to say what the second test checks"
fix_exception: value
stability: preview
ported_from: ["eslint-plugin-playwright/no-identical-title"]
since: 0.5.0
---

Two tests, or two `describe` blocks, in the same block do not share a title. The Playwright Test counterpart of `testing/no-identical-title`, for `test`, `test.describe` and `test.beforeEach`.

**Why:** Reports, `-t` filters and snapshots key on the title; a duplicate makes one test indistinguishable from the other.

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
  any:
    - { has: { pattern: 'it($T, $$$A)' }, follows: { stopBy: end, has: { pattern: 'it($T, $$$B)' } } }
    - { has: { pattern: 'test($T, $$$A)' }, follows: { stopBy: end, has: { pattern: 'test($T, $$$B)' } } }
    - { has: { pattern: 'describe($T, $$$A)' }, follows: { stopBy: end, has: { pattern: 'describe($T, $$$B)' } } }
constraints:
  T: { any: [{ kind: string }, { kind: template_string }] }
message: "Another test or block here has the title $T"
```

## How to fix

1. Rename one of the two to say what it checks that the other does not.
2. If they check the same thing, delete one.

## Examples

### JavaScript

#### Don't

```js
test("adds", () => {});
test("adds", () => {});
```

#### Do

```js
test("adds", () => {});
test("adds negatives", () => {});
```

### JSX

#### Don't

```jsx
test("adds", () => {});
test("adds", () => {});
```

#### Do

```jsx
test("adds", () => {});
test("adds negatives", () => {});
```

### TypeScript

#### Don't

```ts
test("adds", () => {});
test("adds", () => {});
```

#### Do

```ts
test("adds", () => {});
test("adds negatives", () => {});
```

### TSX

#### Don't

```tsx
test("adds", () => {});
test("adds", () => {});
```

#### Do

```tsx
test("adds", () => {});
test("adds negatives", () => {});
```
