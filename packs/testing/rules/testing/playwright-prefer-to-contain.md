---
id: testing/playwright-prefer-to-contain
title: "`toContain` for an `includes` check"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: auto
stability: preview
ported_from: ["eslint-plugin-playwright/prefer-to-contain"]
since: 0.5.0
---

`expect(list.includes(x)).toBe(true)` is written `expect(list).toContain(x)`, and the `false` form `expect(list).not.toContain(x)`. The Playwright Test counterpart of `testing/prefer-to-contain`, for `test`, `test.describe` and `test.beforeEach`.

**Why:** On failure `toContain` prints the list and the value; the boolean form prints only `expected true, received false`.

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
    - all: [{ pattern: "expect($A.includes($B)).$M(true)" }, { pattern: $W0 }]
    - all: [{ pattern: "expect($A.includes($B)).$M(false)" }, { pattern: $W1 }]
    - all: [{ pattern: "expect($A.includes($B)).not.$M(false)" }, { pattern: $W2 }]
    - all: [{ pattern: "expect($A.includes($B)).not.$M(true)" }, { pattern: $W3 }]
constraints:
  M: { regex: '^(toBe|toEqual|toStrictEqual)$' }
message: "Use `toContain` to assert membership"
fix:
  - replace: { target: $W0, with: 'expect($A).toContain($B)' }
  - replace: { target: $W1, with: 'expect($A).not.toContain($B)' }
  - replace: { target: $W2, with: 'expect($A).toContain($B)' }
  - replace: { target: $W3, with: 'expect($A).not.toContain($B)' }
```

## Examples

### JavaScript

#### Don't

```js
test("tags", () => {
  expect(tags.includes("new")).toBe(true);
});
```

#### Do

```js
test("tags", () => {
  expect(tags).toContain("new");
});
```

### JSX

#### Don't

```jsx
test("tags", () => {
  expect(tags.includes("new")).toBe(true);
});
```

#### Do

```jsx
test("tags", () => {
  expect(tags).toContain("new");
});
```

### TypeScript

#### Don't

```ts
test("tags", () => {
  expect(tags.includes("new")).toBe(true);
});
```

#### Do

```ts
test("tags", () => {
  expect(tags).toContain("new");
});
```

### TSX

#### Don't

```tsx
test("tags", () => {
  expect(tags.includes("new")).toBe(true);
});
```

#### Do

```tsx
test("tags", () => {
  expect(tags).toContain("new");
});
```
