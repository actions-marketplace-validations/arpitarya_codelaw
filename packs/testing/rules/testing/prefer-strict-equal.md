---
id: testing/prefer-strict-equal
title: "`toStrictEqual` over `toEqual`"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: suggest
fix_reason: "`toStrictEqual` also checks `undefined` properties, sparse arrays and class types, so an assertion that passed may now fail"
stability: preview
ported_from: ["eslint-plugin-jest/prefer-strict-equal", "@vitest/eslint-plugin/prefer-strict-equal"]
since: 0.5.0
---

Deep equality is asserted with `toStrictEqual`, not `toEqual`.

**Why:** `toEqual` ignores `undefined` properties and object types, so `{ a: undefined }` equals `{}` and a class instance equals a plain object; that hides real differences.

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
  has: { field: function, has: { field: property, regex: '^toEqual$', pattern: $M } }
message: "Use `toStrictEqual`"
fix:
  - replace: { target: $M, with: toStrictEqual }
```

## Examples

### JavaScript

#### Don't

```js
test("user", () => {
  expect(user).toEqual({ id: 1 });
});
```

#### Do

```js
test("user", () => {
  expect(user).toStrictEqual({ id: 1 });
});
```

### JSX

#### Don't

```jsx
test("user", () => {
  expect(user).toEqual({ id: 1 });
});
```

#### Do

```jsx
test("user", () => {
  expect(user).toStrictEqual({ id: 1 });
});
```

### TypeScript

#### Don't

```ts
test("user", () => {
  expect(user).toEqual({ id: 1 });
});
```

#### Do

```ts
test("user", () => {
  expect(user).toStrictEqual({ id: 1 });
});
```

### TSX

#### Don't

```tsx
test("user", () => {
  expect(user).toEqual({ id: 1 });
});
```

#### Do

```tsx
test("user", () => {
  expect(user).toStrictEqual({ id: 1 });
});
```
