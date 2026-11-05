---
id: testing/prefer-to-have-been-called
title: "`not.toHaveBeenCalled()` for zero calls"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: auto
stability: preview
ported_from: ["eslint-plugin-jest/prefer-to-have-been-called", "@vitest/eslint-plugin/prefer-to-have-been-called"]
since: 0.5.0
---

`toHaveBeenCalledTimes(0)` is written `not.toHaveBeenCalled()`.

**Why:** It says what is meant, and the failure message lists the calls that were made.

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
    - all: [{ pattern: "expect($A).toHaveBeenCalledTimes(0)" }, { pattern: $W0 }]
    - all: [{ pattern: "expect($A).not.toHaveBeenCalledTimes(0)" }, { pattern: $W1 }]
message: "Use `not.toHaveBeenCalled()` for a mock that was not called"
fix:
  - replace: { target: $W0, with: 'expect($A).not.toHaveBeenCalled()' }
  - replace: { target: $W1, with: 'expect($A).toHaveBeenCalled()' }
```

## Examples

### JavaScript

#### Don't

```js
test("skips", () => {
  expect(onSave).toHaveBeenCalledTimes(0);
});
```

#### Do

```js
test("skips", () => {
  expect(onSave).not.toHaveBeenCalled();
});
```

### JSX

#### Don't

```jsx
test("skips", () => {
  expect(onSave).toHaveBeenCalledTimes(0);
});
```

#### Do

```jsx
test("skips", () => {
  expect(onSave).not.toHaveBeenCalled();
});
```

### TypeScript

#### Don't

```ts
test("skips", () => {
  expect(onSave).toHaveBeenCalledTimes(0);
});
```

#### Do

```ts
test("skips", () => {
  expect(onSave).not.toHaveBeenCalled();
});
```

### TSX

#### Don't

```tsx
test("skips", () => {
  expect(onSave).toHaveBeenCalledTimes(0);
});
```

#### Do

```tsx
test("skips", () => {
  expect(onSave).not.toHaveBeenCalled();
});
```
