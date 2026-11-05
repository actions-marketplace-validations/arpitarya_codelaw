---
id: testing/valid-title
title: "Test titles are non-empty strings without stray spaces"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: auto
stability: preview
ported_from: ["eslint-plugin-jest/valid-title", "@vitest/eslint-plugin/valid-title"]
since: 0.5.0
---

A test or `describe` title is a string (or a variable holding one), not empty, without leading or trailing spaces, and does not start with its own function's name (`test("test adds")`). The fix trims the spaces; an empty, non-string or repeated-prefix title is reported for a hand edit.

**Why:** The title is how a test is found in a report and selected with `-t`; an empty or padded one breaks both.

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
    - kind: string
      nthChild: 1
      inside: { kind: arguments, inside: { any: [{ matches: test-call }, { matches: describe-call }] } }
      regex: '^(""|'''')$'
    - any: [{ kind: number }, { kind: 'true' }, { kind: 'false' }, { kind: 'null' }, { kind: object }, { kind: array }]
      nthChild: 1
      inside: { kind: arguments, inside: { any: [{ matches: test-call }, { matches: describe-call }] } }
    - kind: string_fragment
      regex: '^\s|\s$'
      pattern: $T
      inside: { kind: string, nthChild: 1, inside: { kind: arguments, inside: { any: [{ matches: test-call }, { matches: describe-call }] } } }
    - kind: string_fragment
      regex: '^(describe|it|test)\b'
      inside: { kind: string, nthChild: 1, inside: { kind: arguments, inside: { kind: call_expression, has: { field: function, regex: '^(describe|it|test)$' } } } }
message: "Fix this title"
fix:
  - replace: { target: $T, with: '$T|trim' }
```

## Examples

### JavaScript

#### Don't

```js
test(" adds two numbers ", () => {
  expect(add(1, 2)).toBe(3);
});
```

#### Do

```js
test("adds two numbers", () => {
  expect(add(1, 2)).toBe(3);
});
```

### JSX

#### Don't

```jsx
test(" adds two numbers ", () => {
  expect(add(1, 2)).toBe(3);
});
```

#### Do

```jsx
test("adds two numbers", () => {
  expect(add(1, 2)).toBe(3);
});
```

### TypeScript

#### Don't

```ts
test(" adds two numbers ", () => {
  expect(add(1, 2)).toBe(3);
});
```

#### Do

```ts
test("adds two numbers", () => {
  expect(add(1, 2)).toBe(3);
});
```

### TSX

#### Don't

```tsx
test(" adds two numbers ", () => {
  expect(add(1, 2)).toBe(3);
});
```

#### Do

```tsx
test("adds two numbers", () => {
  expect(add(1, 2)).toBe(3);
});
```
