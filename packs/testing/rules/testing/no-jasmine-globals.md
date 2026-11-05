---
id: testing/no-jasmine-globals
title: "No Jasmine globals"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: auto
stability: preview
ported_from: ["eslint-plugin-jest/no-jasmine-globals"]
since: 0.5.0
---

A Jest suite does not use Jasmine's globals. `jasmine.any`, `anything`, `objectContaining`, `arrayContaining` and `stringMatching` become `expect.*` (fixed automatically: they are the same matchers); `spyOn`, `fail`, `pending` and other `jasmine.*` are reported for a hand edit, since Jest's counterparts behave differently (`jest.spyOn` calls through; `fail` has no counterpart).

**Why:** Jest's Jasmine compatibility is legacy: the `jest-circus` runner, the default since Jest 27, does not define them.

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
  any:
    - { kind: member_expression, regex: '^jasmine\.any$', pattern: $J0 }
    - { kind: member_expression, regex: '^jasmine\.anything$', pattern: $J1 }
    - { kind: member_expression, regex: '^jasmine\.objectContaining$', pattern: $J2 }
    - { kind: member_expression, regex: '^jasmine\.arrayContaining$', pattern: $J3 }
    - { kind: member_expression, regex: '^jasmine\.stringMatching$', pattern: $J4 }
    - { kind: member_expression, regex: '^jasmine\.', all: [{ not: { regex: '^jasmine\.(any|anything|objectContaining|arrayContaining|stringMatching)$' } }, { not: { inside: { kind: member_expression, field: object } } }] }
    - { kind: call_expression, has: { field: function, kind: identifier, regex: '^(fail|pending|spyOn)$' } }
message: "Jasmine globals are not part of Jest; use Jest's own API"
fix:
  - replace: { target: $J0, with: expect.any }
  - replace: { target: $J1, with: expect.anything }
  - replace: { target: $J2, with: expect.objectContaining }
  - replace: { target: $J3, with: expect.arrayContaining }
  - replace: { target: $J4, with: expect.stringMatching }
```

## Examples

### JavaScript

#### Don't

```js
test("shape", () => {
  expect(user).toEqual(jasmine.objectContaining({ id: 1 }));
});
```

#### Do

```js
test("shape", () => {
  expect(user).toEqual(expect.objectContaining({ id: 1 }));
});
```

### JSX

#### Don't

```jsx
test("shape", () => {
  expect(user).toEqual(jasmine.objectContaining({ id: 1 }));
});
```

#### Do

```jsx
test("shape", () => {
  expect(user).toEqual(expect.objectContaining({ id: 1 }));
});
```

### TypeScript

#### Don't

```ts
test("shape", () => {
  expect(user).toEqual(jasmine.objectContaining({ id: 1 }));
});
```

#### Do

```ts
test("shape", () => {
  expect(user).toEqual(expect.objectContaining({ id: 1 }));
});
```

### TSX

#### Don't

```tsx
test("shape", () => {
  expect(user).toEqual(jasmine.objectContaining({ id: 1 }));
});
```

#### Do

```tsx
test("shape", () => {
  expect(user).toEqual(expect.objectContaining({ id: 1 }));
});
```
