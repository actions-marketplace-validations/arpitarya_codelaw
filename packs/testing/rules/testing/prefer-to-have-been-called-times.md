---
id: testing/prefer-to-have-been-called-times
title: "`toHaveBeenCalledTimes` for a call count"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: auto
stability: preview
ported_from: ["eslint-plugin-jest/prefer-to-have-been-called-times", "@vitest/eslint-plugin/prefer-to-have-been-called-times"]
since: 0.5.0
---

A mock's call count is asserted with `expect(fn).toHaveBeenCalledTimes(n)`, not through `fn.mock.calls`.

**Why:** The matcher states the intent and prints the calls on failure; reading `mock.calls` by hand does neither.

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
    - all: [{ pattern: "expect($F.mock.calls).toHaveLength($N)" }, { pattern: $W0 }]
    - all: [{ pattern: "expect($F.mock.calls).not.toHaveLength($N)" }, { pattern: $W1 }]
message: "Use `toHaveBeenCalledTimes` to count calls"
fix:
  - replace: { target: $W0, with: 'expect($F).toHaveBeenCalledTimes($N)' }
  - replace: { target: $W1, with: 'expect($F).not.toHaveBeenCalledTimes($N)' }
```

## Examples

### JavaScript

#### Don't

```js
test("saves once", () => {
  expect(onSave.mock.calls).toHaveLength(1);
});
```

#### Do

```js
test("saves once", () => {
  expect(onSave).toHaveBeenCalledTimes(1);
});
```

### JSX

#### Don't

```jsx
test("saves once", () => {
  expect(onSave.mock.calls).toHaveLength(1);
});
```

#### Do

```jsx
test("saves once", () => {
  expect(onSave).toHaveBeenCalledTimes(1);
});
```

### TypeScript

#### Don't

```ts
test("saves once", () => {
  expect(onSave.mock.calls).toHaveLength(1);
});
```

#### Do

```ts
test("saves once", () => {
  expect(onSave).toHaveBeenCalledTimes(1);
});
```

### TSX

#### Don't

```tsx
test("saves once", () => {
  expect(onSave.mock.calls).toHaveLength(1);
});
```

#### Do

```tsx
test("saves once", () => {
  expect(onSave).toHaveBeenCalledTimes(1);
});
```
