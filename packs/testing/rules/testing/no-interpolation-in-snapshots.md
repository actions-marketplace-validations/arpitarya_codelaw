---
id: testing/no-interpolation-in-snapshots
title: "No interpolation in an inline snapshot"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: manual
fix_reason: "the snapshot has to hold the literal value; a property matcher replaces the dynamic part"
fix_exception: value
stability: preview
ported_from: ["eslint-plugin-jest/no-interpolation-in-snapshots", "@vitest/eslint-plugin/no-interpolation-in-snapshots"]
since: 0.5.0
---

An inline snapshot (`toMatchInlineSnapshot`, `toThrowErrorMatchingInlineSnapshot`) is a literal; it has no `${…}`.

**Why:** Jest rewrites inline snapshots in place on `-u`, and it cannot write an interpolation back; the value can drift from what was meant.

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
  kind: template_substitution
  inside:
    stopBy: end
    kind: arguments
    inside: { matches: matcher-call, has: { field: function, has: { field: property, regex: '^(toMatchInlineSnapshot|toThrowErrorMatchingInlineSnapshot)$' } } }
message: "Inline snapshots cannot hold interpolation"
```

## How to fix

1. Replace the dynamic part with a property matcher: `toMatchInlineSnapshot({ id: expect.any(String) }, `…`)`.
2. Or write the literal value.

## Examples

### JavaScript

#### Don't

```js
test("user", () => {
  expect(user).toMatchInlineSnapshot(`{ id: ${id} }`);
});
```

#### Do

```js
test("user", () => {
  expect(user).toMatchInlineSnapshot({ id: expect.any(String) }, `{ id: Any<String> }`);
});
```

### JSX

#### Don't

```jsx
test("user", () => {
  expect(user).toMatchInlineSnapshot(`{ id: ${id} }`);
});
```

#### Do

```jsx
test("user", () => {
  expect(user).toMatchInlineSnapshot({ id: expect.any(String) }, `{ id: Any<String> }`);
});
```

### TypeScript

#### Don't

```ts
test("user", () => {
  expect(user).toMatchInlineSnapshot(`{ id: ${id} }`);
});
```

#### Do

```ts
test("user", () => {
  expect(user).toMatchInlineSnapshot({ id: expect.any(String) }, `{ id: Any<String> }`);
});
```

### TSX

#### Don't

```tsx
test("user", () => {
  expect(user).toMatchInlineSnapshot(`{ id: ${id} }`);
});
```

#### Do

```tsx
test("user", () => {
  expect(user).toMatchInlineSnapshot({ id: expect.any(String) }, `{ id: Any<String> }`);
});
```
