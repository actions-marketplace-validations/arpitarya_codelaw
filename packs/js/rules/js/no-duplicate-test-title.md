---
id: js/no-duplicate-test-title
title: "No two tests with the same title in a block"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the title is human-written"
fix_exception: authored
stability: preview
ported_from: [eslint-plugin-sonarjs/no-duplicate-test-title]
since: 0.5.0
---

Two tests in the same block do not have the same title.

**Why:** A failure report names the test by its title; with two alike, nobody knows which one failed.

```codelaw
match:
  kind: expression_statement
  pattern: "$T($S, $$$A);"
  follows: { stopBy: end, pattern: "$T($S, $$$B);" }
constraints:
  T: { regex: '^(it|test|specify)$' }
  S: { any: [{ kind: string }, { kind: template_string }] }
message: "Another test in this block has the title $S"
```

## How to fix

1. Give each test a title that says what it alone checks.

## Examples

### JavaScript

#### Don't

```js
it("saves", () => {});
it("saves", () => {});
```

#### Do

```js
it("saves a new record", () => {});
it("saves an edited record", () => {});
```


### JSX

#### Don't

```jsx
it("saves", () => {});
it("saves", () => {});
```

#### Do

```jsx
it("saves a new record", () => {});
it("saves an edited record", () => {});
```


### TypeScript

#### Don't

```ts
it("saves", () => {});
it("saves", () => {});
```

#### Do

```ts
it("saves a new record", () => {});
it("saves an edited record", () => {});
```


### TSX

#### Don't

```tsx
it("saves", () => {});
it("saves", () => {});
```

#### Do

```tsx
it("saves a new record", () => {});
it("saves an edited record", () => {});
```
