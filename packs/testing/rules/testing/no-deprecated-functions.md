---
id: testing/no-deprecated-functions
title: "No deprecated Jest functions"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: auto
stability: preview
ported_from: ["eslint-plugin-jest/no-deprecated-functions"]
since: 0.5.0
---

Jest's removed and deprecated functions are replaced: `jest.resetModuleRegistry` → `jest.resetModules`, `jest.addMatchers` → `expect.extend`, `require.requireActual` → `jest.requireActual`, `require.requireMock` → `jest.requireMock`, `jest.runTimersToTime` → `jest.advanceTimersByTime`, `jest.genMockFromModule` → `jest.createMockFromModule`.

**Why:** Each was renamed and later removed; a suite that calls one breaks on upgrade.

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
  kind: member_expression
  inside: { kind: call_expression, field: function }
  any:
    - { regex: '^jest\.resetModuleRegistry$', pattern: $D0 }
    - { regex: '^jest\.addMatchers$', pattern: $D1 }
    - { regex: '^require\.requireActual$', pattern: $D2 }
    - { regex: '^require\.requireMock$', pattern: $D3 }
    - { regex: '^jest\.runTimersToTime$', pattern: $D4 }
    - { regex: '^jest\.genMockFromModule$', pattern: $D5 }
message: "`$$MATCH` is deprecated; use its replacement"
fix:
  - replace: { target: $D0, with: jest.resetModules }
  - replace: { target: $D1, with: expect.extend }
  - replace: { target: $D2, with: jest.requireActual }
  - replace: { target: $D3, with: jest.requireMock }
  - replace: { target: $D4, with: jest.advanceTimersByTime }
  - replace: { target: $D5, with: jest.createMockFromModule }
```

## Examples

### JavaScript

#### Don't

```js
beforeEach(() => {
  jest.resetModuleRegistry();
});
```

#### Do

```js
beforeEach(() => {
  jest.resetModules();
});
```

### JSX

#### Don't

```jsx
beforeEach(() => {
  jest.resetModuleRegistry();
});
```

#### Do

```jsx
beforeEach(() => {
  jest.resetModules();
});
```

### TypeScript

#### Don't

```ts
beforeEach(() => {
  jest.resetModuleRegistry();
});
```

#### Do

```ts
beforeEach(() => {
  jest.resetModules();
});
```

### TSX

#### Don't

```tsx
beforeEach(() => {
  jest.resetModuleRegistry();
});
```

#### Do

```tsx
beforeEach(() => {
  jest.resetModules();
});
```
