---
id: testing/prefer-to-be-falsy
title: "`toBeFalsy()` over `toBe(false)`"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: suggest
fix_reason: "`toBeFalsy` also passes for `0`, `\"\"`, `null` and `undefined`, so the assertion gets looser"
stability: preview
ported_from: ["@vitest/eslint-plugin/prefer-to-be-falsy"]
since: 0.5.0
---

`toBe(false)` is written `toBeFalsy()`. Opt-in; the opposite of `prefer-strict-boolean-matchers`.

**Why:** For a check that only cares whether a value is falsy, the named matcher says so.

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
  any:
    - all: [{ pattern: "expect($X).toBe(false)" }, { pattern: $W0 }]
    - all: [{ pattern: "expect($X).not.toBe(false)" }, { pattern: $W1 }]
message: "Use `toBeFalsy()`"
fix:
  - replace: { target: $W0, with: 'expect($X).toBeFalsy()' }
  - replace: { target: $W1, with: 'expect($X).not.toBeFalsy()' }
```

## Examples

### JavaScript

#### Don't

```js
test("empty", () => {
  expect(isEmpty(list)).toBe(false);
});
```

#### Do

```js
test("empty", () => {
  expect(isEmpty(list)).toBeFalsy();
});
```

### JSX

#### Don't

```jsx
test("empty", () => {
  expect(isEmpty(list)).toBe(false);
});
```

#### Do

```jsx
test("empty", () => {
  expect(isEmpty(list)).toBeFalsy();
});
```

### TypeScript

#### Don't

```ts
test("empty", () => {
  expect(isEmpty(list)).toBe(false);
});
```

#### Do

```ts
test("empty", () => {
  expect(isEmpty(list)).toBeFalsy();
});
```

### TSX

#### Don't

```tsx
test("empty", () => {
  expect(isEmpty(list)).toBe(false);
});
```

#### Do

```tsx
test("empty", () => {
  expect(isEmpty(list)).toBeFalsy();
});
```
