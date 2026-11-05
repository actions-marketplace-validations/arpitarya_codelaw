---
id: testing/no-unneeded-async-expect-function
title: "No async wrapper around an awaited promise in `expect`"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: auto
stability: preview
ported_from: ["eslint-plugin-jest/no-unneeded-async-expect-function", "@vitest/eslint-plugin/no-unneeded-async-expect-function"]
since: 0.5.0
---

`expect(async () => await p)` and `expect(async () => { await p; })` pass the promise itself: `expect(p)`.

**Why:** The wrapper adds nothing: `resolves` and `rejects` already await the promise.

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
    - all: [{ pattern: "expect(async () => await $P)" }, { pattern: $W0 }]
    - all: [{ pattern: "expect(async () => { await $P; })" }, { pattern: $W1 }]
message: "Pass the promise to `expect` directly"
fix:
  - replace: { target: $W0, with: 'expect($P)' }
  - replace: { target: $W1, with: 'expect($P)' }
```

## Examples

### JavaScript

#### Don't

```js
test("fails", async () => {
  await expect(async () => await save()).rejects.toThrow();
});
```

#### Do

```js
test("fails", async () => {
  await expect(save()).rejects.toThrow();
});
```

### JSX

#### Don't

```jsx
test("fails", async () => {
  await expect(async () => await save()).rejects.toThrow();
});
```

#### Do

```jsx
test("fails", async () => {
  await expect(save()).rejects.toThrow();
});
```

### TypeScript

#### Don't

```ts
test("fails", async () => {
  await expect(async () => await save()).rejects.toThrow();
});
```

#### Do

```ts
test("fails", async () => {
  await expect(save()).rejects.toThrow();
});
```

### TSX

#### Don't

```tsx
test("fails", async () => {
  await expect(async () => await save()).rejects.toThrow();
});
```

#### Do

```tsx
test("fails", async () => {
  await expect(save()).rejects.toThrow();
});
```
