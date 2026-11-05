---
id: testing/playwright-no-duplicate-hooks
title: "One hook of each kind per block"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: manual
fix_reason: "merging two hooks means choosing the order of their statements"
fix_exception: restructure
stability: preview
ported_from: ["eslint-plugin-playwright/no-duplicate-hooks"]
since: 0.5.0
---

A `describe` block (or the file) has at most one `beforeAll`, `beforeEach`, `afterEach` and `afterAll`. The Playwright Test counterpart of `testing/no-duplicate-hooks`, for `test`, `test.describe` and `test.beforeEach`.

**Why:** Two hooks of one kind run in declaration order, which is easy to miss; one hook keeps the setup in one place.

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
    - { has: { pattern: 'beforeAll($$$A)' }, follows: { stopBy: end, has: { pattern: 'beforeAll($$$B)' } } }
    - { has: { pattern: 'beforeEach($$$A)' }, follows: { stopBy: end, has: { pattern: 'beforeEach($$$B)' } } }
    - { has: { pattern: 'afterAll($$$A)' }, follows: { stopBy: end, has: { pattern: 'afterAll($$$B)' } } }
    - { has: { pattern: 'afterEach($$$A)' }, follows: { stopBy: end, has: { pattern: 'afterEach($$$B)' } } }
message: "This block already has this hook; merge the two"
```

## How to fix

1. Move the second hook's statements into the first, in the order they should run.
2. Delete the second hook.

## Examples

### JavaScript

#### Don't

```js
beforeEach(() => {
  reset();
});
beforeEach(() => {
  seed();
});
```

#### Do

```js
beforeEach(() => {
  reset();
  seed();
});
```

### JSX

#### Don't

```jsx
beforeEach(() => {
  reset();
});
beforeEach(() => {
  seed();
});
```

#### Do

```jsx
beforeEach(() => {
  reset();
  seed();
});
```

### TypeScript

#### Don't

```ts
beforeEach(() => {
  reset();
});
beforeEach(() => {
  seed();
});
```

#### Do

```ts
beforeEach(() => {
  reset();
  seed();
});
```

### TSX

#### Don't

```tsx
beforeEach(() => {
  reset();
});
beforeEach(() => {
  seed();
});
```

#### Do

```tsx
beforeEach(() => {
  reset();
  seed();
});
```
