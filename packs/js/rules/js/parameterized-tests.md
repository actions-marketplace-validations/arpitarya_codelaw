---
id: js/parameterized-tests
title: "One parameterized test for tests that differ only in values"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "collapsing the tests is a rewrite into `it.each`/`test.each` that chooses the table and the names of the parameters"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-sonarjs/parameterized-tests]
since: 0.5.0
---

Three or more tests in one block (a file or a `describe` body) whose callbacks hold at least two statements and are the same code except for up to three literal values — a string, number, boolean, `null` or bigint, each in the same place and of the same kind in every test, and fewer than the statements — are reported at the title of the first, with the count. It looks at the concrete tests of Jest, Vitest, Playwright and Bun (`it`/`test`, `.only`, and the framework's own concrete modifiers: Jest `concurrent` and `failing`, Vitest `concurrent`, `fails` and `sequential`, Playwright `fail` and `slow`), found as globals when Jest or Vitest is in use, or imported from `jest`, `@jest/globals`, `vitest`, `@playwright/test` or `bun:test`; `.each`, `.skip`, `.todo`, `.fixme` and a title that is not a plain string are left out, and so is a file that imports another framework (Cypress, Jasmine, Mocha) for the globals. A framework is in use when the file imports it or the nearest `package.json` lists it. Tests inside the callback of another test are not grouped. Behaviour of eslint-plugin-sonarjs 4.2.2 (read, not copied).

**Why:** Copy-pasted tests that change a number or a string say the same thing three times: one table of cases is shorter, and a new case is one more row.

```codelaw
native: js/parameterized-tests
```

## How to fix

1. Replace the tests with one `it.each([[…], …])(title, callback)` whose table holds the values that differed, and name them as parameters.

## Examples

### JavaScript

#### Don't

```js
import { expect, it } from 'vitest';

it('doubles one', () => {
  const value = double(1);
  expect(value).toBe(2);
  expect(typeof value).toBe('number');
});

it('doubles two', () => {
  const value = double(2);
  expect(value).toBe(4);
  expect(typeof value).toBe('number');
});

it('doubles three', () => {
  const value = double(3);
  expect(value).toBe(6);
  expect(typeof value).toBe('number');
});
```

#### Do

```js
import { expect, it } from 'vitest';

it.each([
  [1, 2],
  [2, 4],
  [3, 6],
])('doubles %i', (input, expected) => {
  const value = double(input);
  expect(value).toBe(expected);
  expect(typeof value).toBe('number');
});
```


### JSX

#### Don't

```jsx
import { expect, it } from 'vitest';

it('doubles one', () => {
  const value = double(1);
  expect(value).toBe(2);
  expect(typeof value).toBe('number');
});

it('doubles two', () => {
  const value = double(2);
  expect(value).toBe(4);
  expect(typeof value).toBe('number');
});

it('doubles three', () => {
  const value = double(3);
  expect(value).toBe(6);
  expect(typeof value).toBe('number');
});
```

#### Do

```jsx
import { expect, it } from 'vitest';

it.each([
  [1, 2],
  [2, 4],
  [3, 6],
])('doubles %i', (input, expected) => {
  const value = double(input);
  expect(value).toBe(expected);
  expect(typeof value).toBe('number');
});
```


### TypeScript

#### Don't

```ts
import { expect, it } from 'vitest';

it('doubles one', () => {
  const value = double(1);
  expect(value).toBe(2);
  expect(typeof value).toBe('number');
});

it('doubles two', () => {
  const value = double(2);
  expect(value).toBe(4);
  expect(typeof value).toBe('number');
});

it('doubles three', () => {
  const value = double(3);
  expect(value).toBe(6);
  expect(typeof value).toBe('number');
});
```

#### Do

```ts
import { expect, it } from 'vitest';

it.each([
  [1, 2],
  [2, 4],
  [3, 6],
])('doubles %i', (input, expected) => {
  const value = double(input);
  expect(value).toBe(expected);
  expect(typeof value).toBe('number');
});
```


### TSX

#### Don't

```tsx
import { expect, it } from 'vitest';

it('doubles one', () => {
  const value = double(1);
  expect(value).toBe(2);
  expect(typeof value).toBe('number');
});

it('doubles two', () => {
  const value = double(2);
  expect(value).toBe(4);
  expect(typeof value).toBe('number');
});

it('doubles three', () => {
  const value = double(3);
  expect(value).toBe(6);
  expect(typeof value).toBe('number');
});
```

#### Do

```tsx
import { expect, it } from 'vitest';

it.each([
  [1, 2],
  [2, 4],
  [3, 6],
])('doubles %i', (input, expected) => {
  const value = double(input);
  expect(value).toBe(expected);
  expect(typeof value).toBe('number');
});
```
