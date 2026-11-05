---
id: testing/prefer-called-once
title: "`toHaveBeenCalledOnce()` over `toHaveBeenCalledTimes(1)`"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: auto
stability: preview
ported_from: ["@vitest/eslint-plugin/prefer-called-once"]
since: 0.5.0
---

`toHaveBeenCalledTimes(1)` is written `toHaveBeenCalledOnce()`. Opt-in; the opposite of `testing/prefer-called-times`.

**Why:** The named matcher says it in a word.

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
  all:
    - has: { field: function, has: { field: property, regex: '^(toHaveBeenCalledTimes|toBeCalledTimes)$', pattern: $M } }
    - has: { field: arguments, regex: '^\(1\)$', pattern: $A }
message: "Use `toHaveBeenCalledOnce()`"
fix:
  - replace: { target: $M, with: toHaveBeenCalledOnce }
  - replace: { target: $A, with: '()' }
```

## Examples

### JavaScript

#### Don't

```js
test("saves", () => {
  expect(save).toHaveBeenCalledTimes(1);
});
```

#### Do

```js
test("saves", () => {
  expect(save).toHaveBeenCalledOnce();
});
```

### JSX

#### Don't

```jsx
test("saves", () => {
  expect(save).toHaveBeenCalledTimes(1);
});
```

#### Do

```jsx
test("saves", () => {
  expect(save).toHaveBeenCalledOnce();
});
```

### TypeScript

#### Don't

```ts
test("saves", () => {
  expect(save).toHaveBeenCalledTimes(1);
});
```

#### Do

```ts
test("saves", () => {
  expect(save).toHaveBeenCalledOnce();
});
```

### TSX

#### Don't

```tsx
test("saves", () => {
  expect(save).toHaveBeenCalledTimes(1);
});
```

#### Do

```tsx
test("saves", () => {
  expect(save).toHaveBeenCalledOnce();
});
```
