---
id: testing/no-alias-methods
title: "Matchers by their canonical names"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: auto
stability: preview
ported_from: ["eslint-plugin-jest/no-alias-methods", "@vitest/eslint-plugin/no-alias-methods"]
since: 0.5.0
---

An assertion uses each matcher's canonical name: `toHaveBeenCalled`, not `toBeCalled`; `toThrow`, not `toThrowError`; and so on for the eleven aliases Jest and Vitest keep for older code. The fix renames the matcher.

**Why:** Two names for one matcher make a suite inconsistent to read and search; the aliases are deprecated and Jest 30 removed them.

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
  has:
    field: function
    has:
      field: property
      any:
        - { regex: '^toBeCalled$', pattern: $A0 }
        - { regex: '^toBeCalledTimes$', pattern: $A1 }
        - { regex: '^toBeCalledWith$', pattern: $A2 }
        - { regex: '^lastCalledWith$', pattern: $A3 }
        - { regex: '^nthCalledWith$', pattern: $A4 }
        - { regex: '^toReturn$', pattern: $A5 }
        - { regex: '^toReturnTimes$', pattern: $A6 }
        - { regex: '^toReturnWith$', pattern: $A7 }
        - { regex: '^lastReturnedWith$', pattern: $A8 }
        - { regex: '^nthReturnedWith$', pattern: $A9 }
        - { regex: '^toThrowError$', pattern: $A10 }
message: "`$$MATCH` uses an alias; use the canonical matcher name"
fix:
  - replace: { target: $A0, with: toHaveBeenCalled }
  - replace: { target: $A1, with: toHaveBeenCalledTimes }
  - replace: { target: $A2, with: toHaveBeenCalledWith }
  - replace: { target: $A3, with: toHaveBeenLastCalledWith }
  - replace: { target: $A4, with: toHaveBeenNthCalledWith }
  - replace: { target: $A5, with: toHaveReturned }
  - replace: { target: $A6, with: toHaveReturnedTimes }
  - replace: { target: $A7, with: toHaveReturnedWith }
  - replace: { target: $A8, with: toHaveLastReturnedWith }
  - replace: { target: $A9, with: toHaveNthReturnedWith }
  - replace: { target: $A10, with: toThrow }
```

## Examples

### JavaScript

#### Don't

```js
test("calls the handler", () => {
  expect(onSave).toBeCalledWith(1);
  expect(() => run()).toThrowError("boom");
});
```

#### Do

```js
test("calls the handler", () => {
  expect(onSave).toHaveBeenCalledWith(1);
  expect(() => run()).toThrow("boom");
});
```

### JSX

#### Don't

```jsx
test("calls the handler", () => {
  expect(onSave).toBeCalledWith(1);
  expect(() => run()).toThrowError("boom");
});
```

#### Do

```jsx
test("calls the handler", () => {
  expect(onSave).toHaveBeenCalledWith(1);
  expect(() => run()).toThrow("boom");
});
```

### TypeScript

#### Don't

```ts
test("calls the handler", () => {
  expect(onSave).toBeCalledWith(1);
  expect(() => run()).toThrowError("boom");
});
```

#### Do

```ts
test("calls the handler", () => {
  expect(onSave).toHaveBeenCalledWith(1);
  expect(() => run()).toThrow("boom");
});
```

### TSX

#### Don't

```tsx
test("calls the handler", () => {
  expect(onSave).toBeCalledWith(1);
  expect(() => run()).toThrowError("boom");
});
```

#### Do

```tsx
test("calls the handler", () => {
  expect(onSave).toHaveBeenCalledWith(1);
  expect(() => run()).toThrow("boom");
});
```
