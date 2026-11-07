---
id: js/no-empty-parameterized-test-dataset
title: "No each() with an empty table"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the cases are the author's"
fix_exception: authored
stability: preview
ported_from: [eslint-plugin-sonarjs/no-empty-parameterized-test-dataset]
since: 0.5.0
---

`it.each`, `test.each` and `describe.each` are not given an empty array: the test then never runs.

**Why:** A test with no cases passes without checking anything.

```codelaw
match:
  pattern: "$T.each([])"
constraints:
  T: { regex: '^(it|test|describe)$' }
message: "This table has no cases, so the test never runs"
```

## How to fix

1. Write the cases into the table, or delete the test.

## Examples

### JavaScript

#### Don't

```js
it.each([])("adds %i", (n) => {});
```

#### Do

```js
it.each([1, 2])("adds %i", (n) => {});
```


### JSX

#### Don't

```jsx
it.each([])("adds %i", (n) => {});
```

#### Do

```jsx
it.each([1, 2])("adds %i", (n) => {});
```


### TypeScript

#### Don't

```ts
it.each([])("adds %i", (n) => {});
```

#### Do

```ts
it.each([1, 2])("adds %i", (n) => {});
```


### TSX

#### Don't

```tsx
it.each([])("adds %i", (n) => {});
```

#### Do

```tsx
it.each([1, 2])("adds %i", (n) => {});
```
