---
id: testing/no-restricted-vi-methods
title: "No `vi` methods the project restricts"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: manual
fix_reason: "what to use instead is the project's convention, named in its message"
fix_exception: value
stability: preview
options: { methods: { type: "call[]", default: [] } }
ported_from: ["@vitest/eslint-plugin/no-restricted-vi-methods"]
since: 0.5.0
---

A call to a `vi.*` method the project lists in `methods` is reported (`methods = ["vi.useFakeTimers"]`). The list is empty by default.

**Why:** A project bans a method when it has a better convention; the rule keeps it out.

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
  has: { field: function, regex: '^vi\.' }
where:
  $$MATCH: { calls: methods }
message: "`$$MATCH` is restricted in this project"
```

## How to fix

1. Replace the call with the project's convention.

## Examples

### JavaScript

#### Don't

```js options={"methods":["vi.useFakeTimers"]}
beforeEach(() => {
  vi.useFakeTimers();
});
```

#### Do

```js options={"methods":["vi.useFakeTimers"]}
beforeEach(() => {
  useClock();
});
```

### JSX

#### Don't

```jsx options={"methods":["vi.useFakeTimers"]}
beforeEach(() => {
  vi.useFakeTimers();
});
```

#### Do

```jsx options={"methods":["vi.useFakeTimers"]}
beforeEach(() => {
  useClock();
});
```

### TypeScript

#### Don't

```ts options={"methods":["vi.useFakeTimers"]}
beforeEach(() => {
  vi.useFakeTimers();
});
```

#### Do

```ts options={"methods":["vi.useFakeTimers"]}
beforeEach(() => {
  useClock();
});
```

### TSX

#### Don't

```tsx options={"methods":["vi.useFakeTimers"]}
beforeEach(() => {
  vi.useFakeTimers();
});
```

#### Do

```tsx options={"methods":["vi.useFakeTimers"]}
beforeEach(() => {
  useClock();
});
```
