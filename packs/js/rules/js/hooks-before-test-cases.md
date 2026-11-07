---
id: js/hooks-before-test-cases
title: "Test hooks come before the tests"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "moving the hook is simple but the author checks the order it implies"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-sonarjs/hooks-before-test-cases]
since: 0.5.0
---

`beforeEach`, `beforeAll`, `afterEach`, `afterAll`, `before` and `after` are written before the tests of their block, not after one.

**Why:** Hooks run around every test of the block whatever their position, so one written after a test misleads the reader about what that test sees.

```codelaw
match:
  kind: expression_statement
  has:
    kind: call_expression
    has: { field: function, regex: '^(beforeEach|beforeAll|afterEach|afterAll|before|after)$' }
  follows:
    stopBy: end
    kind: expression_statement
    has:
      kind: call_expression
      has: { field: function, regex: '^(it|test|specify)(\.(only|skip|concurrent|todo))?$' }
message: "Move this hook above the tests"
```

## How to fix

1. Move the hook to the top of its `describe` block, before the first test.

## Examples

### JavaScript

#### Don't

```js
describe("cart", () => {
  it("adds", () => {});
  beforeEach(() => reset());
});
```

#### Do

```js
describe("cart", () => {
  beforeEach(() => reset());
  it("adds", () => {});
});
```


### JSX

#### Don't

```jsx
describe("cart", () => {
  it("adds", () => {});
  beforeEach(() => reset());
});
```

#### Do

```jsx
describe("cart", () => {
  beforeEach(() => reset());
  it("adds", () => {});
});
```


### TypeScript

#### Don't

```ts
describe("cart", () => {
  it("adds", () => {});
  beforeEach(() => reset());
});
```

#### Do

```ts
describe("cart", () => {
  beforeEach(() => reset());
  it("adds", () => {});
});
```


### TSX

#### Don't

```tsx
describe("cart", () => {
  it("adds", () => {});
  beforeEach(() => reset());
});
```

#### Do

```tsx
describe("cart", () => {
  beforeEach(() => reset());
  it("adds", () => {});
});
```
