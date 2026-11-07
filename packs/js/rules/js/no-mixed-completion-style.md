---
id: js/no-mixed-completion-style
title: "A test callback is async or takes done, not both"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "which completion signal to keep depends on how the body finishes, which only the author knows"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-sonarjs/no-mixed-completion-style]
since: 0.5.0
---

An `async` test or hook callback that also takes a `done` parameter tells the runner two ways that it finished. Jest and Mocha reject the mix or hang on it. Keep one: `async` with `await`, or `done` called from the callback.

**Why:** The test fails or times out for a reason unrelated to the code under test.

```codelaw
match:
  any:
    - kind: arrow_function
    - kind: function_expression
  regex: '^async\b'
  has: { field: parameters, has: { nthChild: 1, any: [{ kind: identifier, regex: '^done$' }, { has: { kind: identifier, regex: '^done$' } }] } }
  inside:
    kind: arguments
    inside:
      kind: call_expression
      has: { field: function, regex: '^(it|test|beforeEach|afterEach|beforeAll|afterAll|before|after)(\.(only|skip))?$' }
message: "Use `async` or `done`, not both"
```

## How to fix

1. With `async`, drop the `done` parameter and its calls; await what the test waits for.
2. With `done`, drop `async` and call `done()` (or `done(error)`) when the work ends.

## Examples

### JavaScript

#### Don't

```js
it("loads config", async (done) => {
  await loadConfig();
  done();
});
```

#### Do

```js
it("loads config", async () => {
  await loadConfig();
});
```


### JSX

#### Don't

```jsx
it("loads config", async (done) => {
  await loadConfig();
  done();
});
```

#### Do

```jsx
it("loads config", async () => {
  await loadConfig();
});
```


### TypeScript

#### Don't

```ts
it("loads config", async (done) => {
  await loadConfig();
  done();
});
```

#### Do

```ts
it("loads config", async () => {
  await loadConfig();
});
```


### TSX

#### Don't

```tsx
it("loads config", async (done) => {
  await loadConfig();
  done();
});
```

#### Do

```tsx
it("loads config", async () => {
  await loadConfig();
});
```
