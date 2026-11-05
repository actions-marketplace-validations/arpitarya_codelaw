---
id: js/no-empty-test-title
title: "A test has a title"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the title is human-written"
fix_exception: authored
stability: preview
ported_from: [eslint-plugin-sonarjs/no-empty-test-title]
since: 0.5.0
---

A test or suite is not given an empty title.

**Why:** The title is what a failing run prints; an empty one says nothing.

```codelaw
match:
  kind: call_expression
  has:
    field: function
    regex: ^(it|test|describe|suite|specify|context)(\.(only|skip|each))?$
  all:
  - has:
      field: arguments
      has:
        nthChild: 1
        kind: string
        regex: ^.\s*.$
message: Give the test a title that says what it checks
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JavaScript

#### Don't

```js
it("", () => {
  check();
});
```

#### Do

```js
it("rejects an empty name", () => {
  check();
});
```

### JSX

#### Don't

```jsx
it("", () => {
  check();
});
```

#### Do

```jsx
it("rejects an empty name", () => {
  check();
});
```

### TypeScript

#### Don't

```ts
it("", () => {
  check();
});
```

#### Do

```ts
it("rejects an empty name", () => {
  check();
});
```

### TSX

#### Don't

```tsx
it("", () => {
  check();
});
```

#### Do

```tsx
it("rejects an empty name", () => {
  check();
});
```
