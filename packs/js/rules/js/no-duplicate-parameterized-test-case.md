---
id: js/no-duplicate-parameterized-test-case
title: "No repeated row in an `each()` table"
languages: [javascript, jsx, typescript, tsx]
fix: auto
fix_reason: "a repeated row is removed with the comma that joined it, or its whole line; a row with a comment inside is reported without a fix"
stability: preview
ported_from: [eslint-plugin-sonarjs/no-duplicate-parameterized-test-case]
since: 0.5.0
---

`it.each([...])(name, fn)`, `test.each`, `describe.each` (also with `.only`, `.skip`, `.concurrent`, `.fails`) whose table, an array literal, holds the same row twice — the same code, whatever the spacing or comments — is reported at the first repeat of each row; the fix removes every repeat of it. Spread elements are skipped. Upstream runs only where a test framework (Jest, Vitest, Bun, Playwright) is imported or a dependency; this rule has no such gate, the `it`/`test`/`describe` `.each([…])(name, fn)` shape being specific to them. Behaviour of eslint-plugin-sonarjs 4.2.2 (read, not copied).

**Why:** A row repeated in a table runs the same case again and says nothing new, and usually means the row meant to be different.

```codelaw
native: js/no-duplicate-parameterized-test-case
```

## Examples

### JavaScript

#### Don't

```js
it.each([
  [1, 2],
  [3, 4],
  [1, 2],
])('adds', (a, b) => {});
```

#### Do

```js
it.each([
  [1, 2],
  [3, 4],
])('adds', (a, b) => {});
```

#### Fixed

```js
it.each([
  [1, 2],
  [3, 4],
])('adds', (a, b) => {});
```


### JSX

#### Don't

```jsx
it.each([
  [1, 2],
  [3, 4],
  [1, 2],
])('adds', (a, b) => {});
```

#### Do

```jsx
it.each([
  [1, 2],
  [3, 4],
])('adds', (a, b) => {});
```

#### Fixed

```jsx
it.each([
  [1, 2],
  [3, 4],
])('adds', (a, b) => {});
```


### TypeScript

#### Don't

```ts
it.each([
  [1, 2],
  [3, 4],
  [1, 2],
])('adds', (a, b) => {});
```

#### Do

```ts
it.each([
  [1, 2],
  [3, 4],
])('adds', (a, b) => {});
```

#### Fixed

```ts
it.each([
  [1, 2],
  [3, 4],
])('adds', (a, b) => {});
```


### TSX

#### Don't

```tsx
it.each([
  [1, 2],
  [3, 4],
  [1, 2],
])('adds', (a, b) => {});
```

#### Do

```tsx
it.each([
  [1, 2],
  [3, 4],
])('adds', (a, b) => {});
```

#### Fixed

```tsx
it.each([
  [1, 2],
  [3, 4],
])('adds', (a, b) => {});
```
