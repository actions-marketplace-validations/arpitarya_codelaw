---
id: testing/playwright-prefer-to-have-length
title: "`toHaveLength` for a length"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: auto
stability: preview
ported_from: ["eslint-plugin-playwright/prefer-to-have-length"]
since: 0.5.0
---

An assertion on `x.length` uses `expect(x).toHaveLength(n)`, not `expect(x.length).toBe(n)` (or `toEqual`, `toStrictEqual`). The Playwright Test counterpart of `testing/prefer-to-have-length`, for `test`, `test.describe` and `test.beforeEach`.

**Why:** `toHaveLength` reports the value itself when it fails, not just two numbers, and says what is being checked.

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
    - all: [{ pattern: "expect($X.length).$M($N)" }, { pattern: $W0 }]
    - all: [{ pattern: "expect($X.length).not.$M($N)" }, { pattern: $W1 }]
constraints:
  M: { regex: '^(toBe|toEqual|toStrictEqual)$' }
message: "Use `toHaveLength` to assert a length"
fix:
  - replace: { target: $W0, with: 'expect($X).toHaveLength($N)' }
  - replace: { target: $W1, with: 'expect($X).not.toHaveLength($N)' }
```

## Examples

### JavaScript

#### Don't

```js
test("lists", () => {
  expect(items.length).toBe(3);
});
```

#### Do

```js
test("lists", () => {
  expect(items).toHaveLength(3);
});
```

### JSX

#### Don't

```jsx
test("lists", () => {
  expect(items.length).toBe(3);
});
```

#### Do

```jsx
test("lists", () => {
  expect(items).toHaveLength(3);
});
```

### TypeScript

#### Don't

```ts
test("lists", () => {
  expect(items.length).toBe(3);
});
```

#### Do

```ts
test("lists", () => {
  expect(items).toHaveLength(3);
});
```

### TSX

#### Don't

```tsx
test("lists", () => {
  expect(items.length).toBe(3);
});
```

#### Do

```tsx
test("lists", () => {
  expect(items).toHaveLength(3);
});
```
