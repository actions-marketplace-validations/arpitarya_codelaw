---
id: testing/prefer-to-be-object
title: "`toBeInstanceOf(Object)` over an `instanceof` check"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: auto
stability: preview
ported_from: ["@vitest/eslint-plugin/prefer-to-be-object"]
since: 0.5.0
---

`expect(x instanceof Object).toBe(true)` is written `expect(x).toBeInstanceOf(Object)` (and `false` as `not.toBeInstanceOf`).

**Why:** The matcher prints the value on failure; the boolean form prints only `false`.

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
    - all: [{ pattern: "expect($X instanceof Object).toBe(true)" }, { pattern: $W0 }]
    - all: [{ pattern: "expect($X instanceof Object).toBe(false)" }, { pattern: $W1 }]
message: "Use `toBeInstanceOf(Object)`"
fix:
  - replace: { target: $W0, with: 'expect($X).toBeInstanceOf(Object)' }
  - replace: { target: $W1, with: 'expect($X).not.toBeInstanceOf(Object)' }
```

## Examples

### JavaScript

#### Don't

```js
test("shape", () => {
  expect(config instanceof Object).toBe(true);
});
```

#### Do

```js
test("shape", () => {
  expect(config).toBeInstanceOf(Object);
});
```

### JSX

#### Don't

```jsx
test("shape", () => {
  expect(config instanceof Object).toBe(true);
});
```

#### Do

```jsx
test("shape", () => {
  expect(config).toBeInstanceOf(Object);
});
```

### TypeScript

#### Don't

```ts
test("shape", () => {
  expect(config instanceof Object).toBe(true);
});
```

#### Do

```ts
test("shape", () => {
  expect(config).toBeInstanceOf(Object);
});
```

### TSX

#### Don't

```tsx
test("shape", () => {
  expect(config instanceof Object).toBe(true);
});
```

#### Do

```tsx
test("shape", () => {
  expect(config).toBeInstanceOf(Object);
});
```
