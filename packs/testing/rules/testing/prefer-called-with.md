---
id: testing/prefer-called-with
title: "`toHaveBeenCalledWith` over `toHaveBeenCalled`"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: manual
fix_reason: "the expected arguments are the author's to write"
fix_exception: value
stability: preview
ported_from: ["eslint-plugin-jest/prefer-called-with", "@vitest/eslint-plugin/prefer-called-with"]
since: 0.5.0
---

A call is asserted with its arguments, `toHaveBeenCalledWith(…)`, not only `toHaveBeenCalled()`. Negated forms are left alone.

**Why:** Checking the arguments catches a call made with the wrong values, which `toHaveBeenCalled` passes.

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
  has: { field: function, all: [{ has: { field: property, regex: '^(toBeCalled|toHaveBeenCalled)$' } }, { not: { regex: '\.not\.' } }] }
message: "Assert the arguments with `toHaveBeenCalledWith`"
```

## How to fix

1. Replace `toHaveBeenCalled()` with `toHaveBeenCalledWith(<the expected arguments>)`.

## Examples

### JavaScript

#### Don't

```js
test("saves", () => {
  expect(save).toHaveBeenCalled();
});
```

#### Do

```js
test("saves", () => {
  expect(save).toHaveBeenCalledWith({ id: 1 });
});
```

### JSX

#### Don't

```jsx
test("saves", () => {
  expect(save).toHaveBeenCalled();
});
```

#### Do

```jsx
test("saves", () => {
  expect(save).toHaveBeenCalledWith({ id: 1 });
});
```

### TypeScript

#### Don't

```ts
test("saves", () => {
  expect(save).toHaveBeenCalled();
});
```

#### Do

```ts
test("saves", () => {
  expect(save).toHaveBeenCalledWith({ id: 1 });
});
```

### TSX

#### Don't

```tsx
test("saves", () => {
  expect(save).toHaveBeenCalled();
});
```

#### Do

```tsx
test("saves", () => {
  expect(save).toHaveBeenCalledWith({ id: 1 });
});
```
