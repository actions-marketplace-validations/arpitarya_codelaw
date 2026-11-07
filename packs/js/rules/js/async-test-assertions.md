---
id: js/async-test-assertions
title: "Await an async assertion in a test"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "awaiting the assertion makes the test wait for it; a test that relied on running on meanwhile needs review"
stability: preview
ported_from: [eslint-plugin-sonarjs/async-test-assertions]
since: 0.5.0
---

`expect(promise).resolves…`, `expect(promise).rejects…`, `assert.rejects(…)` and `assert.doesNotReject(…)` return a promise. As a bare statement in a test callback the test ends before it settles, so a failure is never reported. The assertion is awaited, and a callback that is not `async` becomes one. Callbacks that take a `done` parameter are left.

**Why:** The test passes whatever the promise does.

```codelaw
utils:
  async_assertion:
    kind: expression_statement
    any:
      - has: { kind: call_expression, has: { stopBy: end, kind: member_expression, regex: '^expect\([\s\S]*\)\s*\.\s*(resolves|rejects)$' } }
      - has: { kind: call_expression, has: { field: function, regex: '^assert\.(rejects|doesNotReject)$' } }
  test_call:
    kind: call_expression
    has: { field: function, regex: '^(it|test|beforeEach|afterEach|beforeAll|afterAll|before|after)(\.(only|skip))?$' }
match:
  matches: async_assertion
  any:
    - all:
        - inside:
            kind: statement_block
            inside:
              any: [{ kind: arrow_function }, { kind: function_expression }]
              regex: '^async\b'
              inside: { kind: arguments, inside: { matches: test_call } }
              not: { has: { field: parameters, has: { kind: identifier, regex: '^done$' } } }
        - pattern: $S1
    - all:
        - inside:
            kind: statement_block
            inside:
              any: [{ kind: arrow_function }, { kind: function_expression }]
              not: { any: [{ regex: '^async\b' }, { has: { field: parameters, has: { kind: identifier, regex: '^done$' } } }] }
              inside: { kind: arguments, inside: { matches: test_call } }
              pattern: $CB
        - pattern: $S2
message: "Await this assertion, or the test ends before it settles"
fix:
  - insert-before: { target: $S1, text: "await " }
  - insert-before: { target: $S2, text: "await " }
  - insert-before: { target: $CB, text: "async " }
```

## Examples

### JavaScript

#### Don't

```js
it("loads the user", () => {
  expect(fetchUser()).resolves.toBeDefined();
});
```

#### Do

```js
it("loads the user", async () => {
  await expect(fetchUser()).resolves.toBeDefined();
});
```


### JSX

#### Don't

```jsx
it("loads the user", () => {
  expect(fetchUser()).resolves.toBeDefined();
});
```

#### Do

```jsx
it("loads the user", async () => {
  await expect(fetchUser()).resolves.toBeDefined();
});
```


### TypeScript

#### Don't

```ts
it("loads the user", () => {
  expect(fetchUser()).resolves.toBeDefined();
});
```

#### Do

```ts
it("loads the user", async () => {
  await expect(fetchUser()).resolves.toBeDefined();
});
```


### TSX

#### Don't

```tsx
it("loads the user", () => {
  expect(fetchUser()).resolves.toBeDefined();
});
```

#### Do

```tsx
it("loads the user", async () => {
  await expect(fetchUser()).resolves.toBeDefined();
});
```
