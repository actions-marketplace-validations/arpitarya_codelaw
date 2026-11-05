---
id: testing/playwright-prefer-to-be
title: "`toBe` for primitives, `toBeNull`, `toBeUndefined`, `toBeNaN`"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: auto
stability: preview
ported_from: ["eslint-plugin-playwright/prefer-to-be"]
since: 0.5.0
---

A comparison with a number, string or boolean literal uses `toBe`, not `toEqual` or `toStrictEqual`; and `null`, `undefined` and `NaN` have their own matchers: `toBeNull()`, `toBeUndefined()`, `toBeNaN()`. The Playwright Test counterpart of `testing/prefer-to-be`, for `test`, `test.describe` and `test.beforeEach`.

**Why:** For a primitive, `toBe` is the exact check and reads as one; the dedicated matchers say what is expected and print better failures.

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
      - has: { field: function, has: { field: property, regex: '^(toEqual|toStrictEqual)$', pattern: $P } }
      - has: { field: arguments, all: [{ has: { nthChild: 1, any: [{ kind: number }, { kind: string }, { kind: 'true' }, { kind: 'false' }, { kind: template_string, not: { has: { kind: template_substitution } } }] } }, { not: { has: { nthChild: 2 } } }] }
    - all:
      - has: { field: function, has: { field: property, regex: '^(toBe|toEqual|toStrictEqual)$', pattern: $N } }
      - has: { field: arguments, pattern: $NA, all: [{ has: { nthChild: 1, kind: 'null' } }, { not: { has: { nthChild: 2 } } }] }
    - all:
      - has: { field: function, has: { field: property, regex: '^(toBe|toEqual|toStrictEqual)$', pattern: $U } }
      - has: { field: arguments, pattern: $UA, all: [{ has: { nthChild: 1, kind: undefined } }, { not: { has: { nthChild: 2 } } }] }
    - all:
      - has: { field: function, has: { field: property, regex: '^(toBe|toEqual|toStrictEqual)$', pattern: $X } }
      - has: { field: arguments, pattern: $XA, all: [{ has: { nthChild: 1, regex: '^NaN$', kind: identifier } }, { not: { has: { nthChild: 2 } } }] }
message: "Use the matcher made for this value"
fix:
  - replace: { target: $P, with: toBe }
  - replace: { target: $N, with: toBeNull }
  - replace: { target: $NA, with: '()' }
  - replace: { target: $U, with: toBeUndefined }
  - replace: { target: $UA, with: '()' }
  - replace: { target: $X, with: toBeNaN }
  - replace: { target: $XA, with: '()' }
```

## Examples

### JavaScript

#### Don't

```js
test("parses", () => {
  expect(parse("1")).toEqual(1);
  expect(find("x")).toBe(null);
});
```

#### Do

```js
test("parses", () => {
  expect(parse("1")).toBe(1);
  expect(find("x")).toBeNull();
});
```

### JSX

#### Don't

```jsx
test("parses", () => {
  expect(parse("1")).toEqual(1);
  expect(find("x")).toBe(null);
});
```

#### Do

```jsx
test("parses", () => {
  expect(parse("1")).toBe(1);
  expect(find("x")).toBeNull();
});
```

### TypeScript

#### Don't

```ts
test("parses", () => {
  expect(parse("1")).toEqual(1);
  expect(find("x")).toBe(null);
});
```

#### Do

```ts
test("parses", () => {
  expect(parse("1")).toBe(1);
  expect(find("x")).toBeNull();
});
```

### TSX

#### Don't

```tsx
test("parses", () => {
  expect(parse("1")).toEqual(1);
  expect(find("x")).toBe(null);
});
```

#### Do

```tsx
test("parses", () => {
  expect(parse("1")).toBe(1);
  expect(find("x")).toBeNull();
});
```
