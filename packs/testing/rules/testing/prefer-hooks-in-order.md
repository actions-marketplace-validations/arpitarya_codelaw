---
id: testing/prefer-hooks-in-order
title: "Hooks in the order they run"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: manual
fix_reason: "reordering hooks moves their statements; the author checks nothing depends on the old order"
fix_exception: restructure
stability: preview
ported_from: ["eslint-plugin-jest/prefer-hooks-in-order", "@vitest/eslint-plugin/prefer-hooks-in-order"]
since: 0.5.0
---

In a block, hooks are declared in the order Jest runs them: `beforeAll`, `beforeEach`, `afterEach`, `afterAll`.

**Why:** Hooks run in that order whatever their position; declaring them out of order makes the file mislead its reader.

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
    - { has: { pattern: 'beforeAll($$$A)' }, follows: { stopBy: end, has: { regex: '^(beforeEach|afterEach|afterAll)\(' } } }
    - { has: { pattern: 'beforeEach($$$A)' }, follows: { stopBy: end, has: { regex: '^(afterEach|afterAll)\(' } } }
    - { has: { pattern: 'afterEach($$$A)' }, follows: { stopBy: end, has: { regex: '^(afterAll)\(' } } }
message: "Hooks run in the order beforeAll, beforeEach, afterEach, afterAll; declare them in that order"
```

## How to fix

1. Move the hook above the hooks that run after it.

## Examples

### JavaScript

#### Don't

```js
afterEach(() => {
  cleanup();
});
beforeEach(() => {
  setup();
});
```

#### Do

```js
beforeEach(() => {
  setup();
});
afterEach(() => {
  cleanup();
});
```

### JSX

#### Don't

```jsx
afterEach(() => {
  cleanup();
});
beforeEach(() => {
  setup();
});
```

#### Do

```jsx
beforeEach(() => {
  setup();
});
afterEach(() => {
  cleanup();
});
```

### TypeScript

#### Don't

```ts
afterEach(() => {
  cleanup();
});
beforeEach(() => {
  setup();
});
```

#### Do

```ts
beforeEach(() => {
  setup();
});
afterEach(() => {
  cleanup();
});
```

### TSX

#### Don't

```tsx
afterEach(() => {
  cleanup();
});
beforeEach(() => {
  setup();
});
```

#### Do

```tsx
beforeEach(() => {
  setup();
});
afterEach(() => {
  cleanup();
});
```
