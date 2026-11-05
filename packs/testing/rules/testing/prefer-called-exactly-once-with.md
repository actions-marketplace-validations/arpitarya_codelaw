---
id: testing/prefer-called-exactly-once-with
title: "`toHaveBeenCalledExactlyOnceWith`"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: auto
stability: preview
ported_from: ["@vitest/eslint-plugin/prefer-called-exactly-once-with"]
since: 0.5.0
---

Two adjacent assertions on one mock, `toHaveBeenCalledOnce()` and `toHaveBeenCalledWith(…)`, are one: `toHaveBeenCalledExactlyOnceWith(…)`.

**Why:** The single matcher checks both at once and reads as the intent; the pair can drift apart in an edit.

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
  kind: expression_statement
  any:
    - all:
      - pattern: 'expect($F).toHaveBeenCalledOnce();'
      - precedes: { all: [{ pattern: 'expect($F).toHaveBeenCalledWith($$$A);' }, { pattern: $S2 }] }
      - pattern: $S1
    - all:
      - pattern: 'expect($F).toHaveBeenCalledWith($$$A);'
      - precedes: { all: [{ pattern: 'expect($F).toHaveBeenCalledOnce();' }, { pattern: $T2 }] }
      - pattern: $T1
message: "Use `toHaveBeenCalledExactlyOnceWith`"
fix:
  - replace: { target: $S1, with: 'expect($F).toHaveBeenCalledExactlyOnceWith($$$A);' }
  - remove: $S2
  - replace: { target: $T1, with: 'expect($F).toHaveBeenCalledExactlyOnceWith($$$A);' }
  - remove: $T2
```

## Examples

### JavaScript

#### Don't

```js
test("saves", () => {
  expect(save).toHaveBeenCalledOnce();
  expect(save).toHaveBeenCalledWith(user);
});
```

#### Do

```js
test("saves", () => {
  expect(save).toHaveBeenCalledExactlyOnceWith(user);
});
```

### JSX

#### Don't

```jsx
test("saves", () => {
  expect(save).toHaveBeenCalledOnce();
  expect(save).toHaveBeenCalledWith(user);
});
```

#### Do

```jsx
test("saves", () => {
  expect(save).toHaveBeenCalledExactlyOnceWith(user);
});
```

### TypeScript

#### Don't

```ts
test("saves", () => {
  expect(save).toHaveBeenCalledOnce();
  expect(save).toHaveBeenCalledWith(user);
});
```

#### Do

```ts
test("saves", () => {
  expect(save).toHaveBeenCalledExactlyOnceWith(user);
});
```

### TSX

#### Don't

```tsx
test("saves", () => {
  expect(save).toHaveBeenCalledOnce();
  expect(save).toHaveBeenCalledWith(user);
});
```

#### Do

```tsx
test("saves", () => {
  expect(save).toHaveBeenCalledExactlyOnceWith(user);
});
```
