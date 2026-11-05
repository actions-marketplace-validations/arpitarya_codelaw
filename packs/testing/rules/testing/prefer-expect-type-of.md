---
id: testing/prefer-expect-type-of
title: "`expectTypeOf` for a type check"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: suggest
fix_reason: "`expectTypeOf` checks the static type at compile time; the runtime `typeof` check stops running, so the author confirms"
stability: preview
ported_from: ["@vitest/eslint-plugin/prefer-expect-type-of"]
since: 0.5.0
---

`expect(typeof x).toBe("string")` is written `expectTypeOf(x).toBeString()` (and likewise for the other `typeof` names). The suggested edit makes the change.

**Why:** The `typeof` form asserts at run time what the type checker already knows; `expectTypeOf` states it as a type test.

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
    - all: [{ pattern: 'expect(typeof $X).toBe("string")' }, { pattern: $W0 }]
    - all: [{ pattern: 'expect(typeof $X).toBe("number")' }, { pattern: $W1 }]
    - all: [{ pattern: 'expect(typeof $X).toBe("boolean")' }, { pattern: $W2 }]
    - all: [{ pattern: 'expect(typeof $X).toBe("function")' }, { pattern: $W3 }]
    - all: [{ pattern: 'expect(typeof $X).toBe("object")' }, { pattern: $W4 }]
    - all: [{ pattern: 'expect(typeof $X).toBe("undefined")' }, { pattern: $W5 }]
    - all: [{ pattern: 'expect(typeof $X).toBe("symbol")' }, { pattern: $W6 }]
    - all: [{ pattern: 'expect(typeof $X).toBe("bigint")' }, { pattern: $W7 }]
message: "Use `expectTypeOf`"
fix:
  - replace: { target: $W0, with: 'expectTypeOf($X).toBeString()' }
  - replace: { target: $W1, with: 'expectTypeOf($X).toBeNumber()' }
  - replace: { target: $W2, with: 'expectTypeOf($X).toBeBoolean()' }
  - replace: { target: $W3, with: 'expectTypeOf($X).toBeFunction()' }
  - replace: { target: $W4, with: 'expectTypeOf($X).toBeObject()' }
  - replace: { target: $W5, with: 'expectTypeOf($X).toBeUndefined()' }
  - replace: { target: $W6, with: 'expectTypeOf($X).toBeSymbol()' }
  - replace: { target: $W7, with: 'expectTypeOf($X).toBeBigInt()' }
```

## Examples

### JavaScript

#### Don't

```js
test("types", () => {
  expect(typeof id).toBe("string");
});
```

#### Do

```js
test("types", () => {
  expectTypeOf(id).toBeString();
});
```

### JSX

#### Don't

```jsx
test("types", () => {
  expect(typeof id).toBe("string");
});
```

#### Do

```jsx
test("types", () => {
  expectTypeOf(id).toBeString();
});
```

### TypeScript

#### Don't

```ts
test("types", () => {
  expect(typeof id).toBe("string");
});
```

#### Do

```ts
test("types", () => {
  expectTypeOf(id).toBeString();
});
```

### TSX

#### Don't

```tsx
test("types", () => {
  expect(typeof id).toBe("string");
});
```

#### Do

```tsx
test("types", () => {
  expectTypeOf(id).toBeString();
});
```
