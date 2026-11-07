---
id: js/assertions-in-test-cases
title: "A test makes an assertion"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "what the test should check is the author's"
fix_exception: authored
stability: preview
ported_from: [eslint-plugin-sonarjs/assertions-in-test-cases]
since: 0.5.0
---

A test (`it`, `test`, `specify`) given a function calls something that asserts: `expect`, `assert`, a `should` chain, `sinon.assert` or a `cy` command.

**Why:** A test that asserts nothing passes whatever the code does.

```codelaw
match:
  pattern: "$T($S, $F)"
constraints:
  T: { regex: '^(it|test|specify)(\.(only|concurrent))?$' }
  F:
    any: [{ kind: arrow_function }, { kind: function_expression }]
    not:
      has:
        stopBy: end
        any:
          - kind: identifier
            regex: '^(expect|assert|expectTypeOf|assertType)$'
          - kind: property_identifier
            regex: '^(should|expect|assert)$'
          - kind: identifier
            regex: '^cy$'
          - kind: identifier
            regex: '^done$'
message: "This test asserts nothing"
```

## How to fix

1. Add the assertion the test is named for, or delete the test.

## Examples

### JavaScript

#### Don't

```js
it("saves the user", () => {
  save(user);
});
```

#### Do

```js
it("saves the user", () => {
  save(user);
  expect(store.get(user.id)).toEqual(user);
});
```


### JSX

#### Don't

```jsx
it("saves the user", () => {
  save(user);
});
```

#### Do

```jsx
it("saves the user", () => {
  save(user);
  expect(store.get(user.id)).toEqual(user);
});
```


### TypeScript

#### Don't

```ts
it("saves the user", () => {
  save(user);
});
```

#### Do

```ts
it("saves the user", () => {
  save(user);
  expect(store.get(user.id)).toEqual(user);
});
```


### TSX

#### Don't

```tsx
it("saves the user", () => {
  save(user);
});
```

#### Do

```tsx
it("saves the user", () => {
  save(user);
  expect(store.get(user.id)).toEqual(user);
});
```
