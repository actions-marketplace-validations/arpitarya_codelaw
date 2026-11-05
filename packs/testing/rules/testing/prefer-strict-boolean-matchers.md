---
id: testing/prefer-strict-boolean-matchers
title: "`toBe(true)` over `toBeTruthy()`"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: suggest
fix_reason: "`toBe(true)` fails for truthy values other than `true` (`1`, `\"x\"`), so an assertion that passed may now fail"
stability: preview
ported_from: ["@vitest/eslint-plugin/prefer-strict-boolean-matchers"]
since: 0.5.0
---

`toBeTruthy()` and `toBeFalsy()` are written `toBe(true)` and `toBe(false)`. Opt-in; the opposite of `prefer-to-be-truthy` and `prefer-to-be-falsy`.

**Why:** The loose matchers pass for `1`, `""` or `null`, which hides a value of the wrong type.

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
  any:
    - all:
      - has: { field: function, has: { field: property, regex: '^toBeTruthy$', pattern: $T } }
      - has: { field: arguments, regex: '^\(\)$', pattern: $TA }
    - all:
      - has: { field: function, has: { field: property, regex: '^toBeFalsy$', pattern: $F } }
      - has: { field: arguments, regex: '^\(\)$', pattern: $FA }
message: "Use `toBe(true)` or `toBe(false)`"
fix:
  - replace: { target: $T, with: toBe }
  - replace: { target: $TA, with: '(true)' }
  - replace: { target: $F, with: toBe }
  - replace: { target: $FA, with: '(false)' }
```

## Examples

### JavaScript

#### Don't

```js
test("valid", () => {
  expect(isValid(x)).toBeTruthy();
});
```

#### Do

```js
test("valid", () => {
  expect(isValid(x)).toBe(true);
});
```

### JSX

#### Don't

```jsx
test("valid", () => {
  expect(isValid(x)).toBeTruthy();
});
```

#### Do

```jsx
test("valid", () => {
  expect(isValid(x)).toBe(true);
});
```

### TypeScript

#### Don't

```ts
test("valid", () => {
  expect(isValid(x)).toBeTruthy();
});
```

#### Do

```ts
test("valid", () => {
  expect(isValid(x)).toBe(true);
});
```

### TSX

#### Don't

```tsx
test("valid", () => {
  expect(isValid(x)).toBeTruthy();
});
```

#### Do

```tsx
test("valid", () => {
  expect(isValid(x)).toBe(true);
});
```
