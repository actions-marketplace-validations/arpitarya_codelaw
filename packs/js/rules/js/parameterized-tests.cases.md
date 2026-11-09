# js/parameterized-tests — cases

### case: three tests with two values changing

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

```expect
3:4 js/parameterized-tests
```

### case: only two tests

```js
import { expect, it } from 'vitest';

it('a', () => {
  const v = f(1);
  expect(v).toBe(2);
  expect(g(v)).toBe(true);
});
it('b', () => {
  const v = f(2);
  expect(v).toBe(3);
  expect(g(v)).toBe(true);
});
```

```expect
(none)
```

### case: no framework in use

```js
it('a', () => {
  const v = f(1);
  expect(v).toBe(2);
  expect(g(v)).toBe(true);
});
it('b', () => {
  const v = f(2);
  expect(v).toBe(3);
  expect(g(v)).toBe(true);
});
it('c', () => {
  const v = f(3);
  expect(v).toBe(4);
  expect(g(v)).toBe(true);
});
```

```expect
(none)
```

### case: the globals of a framework imported for something else

```js
import { jest } from '@jest/globals';

test('a', () => {
  const v = f(1);
  expect(v).toBe(2);
  expect(g(v)).toBe(true);
});
test('b', () => {
  const v = f(2);
  expect(v).toBe(3);
  expect(g(v)).toBe(true);
});
test('c', () => {
  const v = f(3);
  expect(v).toBe(4);
  expect(g(v)).toBe(true);
});
```

```expect
3:6 js/parameterized-tests
```

### case: the code differs in shape

```js
import { expect, it } from 'vitest';

it('a', () => {
  const v = f(1);
  expect(v).toBe(2);
  expect(g(v)).toBe(true);
});
it('b', () => {
  const v = f(2);
  expect(v).toBe(3);
  expect(h(v)).toBe(true);
});
it('c', () => {
  const v = f(3);
  expect(v).toBe(4);
  expect(g(v)).toBe(true);
});
```

```expect
(none)
```

### case: more than three values change

```js
import { expect, it } from 'vitest';

it('a', () => {
  const v = f(1, 2, 3, 4);
  expect(v).toBe(0);
  expect(g(v)).toBe(true);
});
it('b', () => {
  const v = f(5, 6, 7, 8);
  expect(v).toBe(0);
  expect(g(v)).toBe(true);
});
it('c', () => {
  const v = f(9, 10, 11, 12);
  expect(v).toBe(0);
  expect(g(v)).toBe(true);
});
```

```expect
(none)
```

### case: as many statements as values

```js
import { expect, it } from 'vitest';

it('a', () => {
  expect(f(1)).toBe(2);
  expect(g(1)).toBe(2);
});
it('b', () => {
  expect(f(3)).toBe(4);
  expect(g(5)).toBe(6);
});
it('c', () => {
  expect(f(7)).toBe(8);
  expect(g(9)).toBe(10);
});
```

```expect
(none)
```

### case: a literal of another kind

```js
import { expect, it } from 'vitest';

it('a', () => {
  const v = f(1);
  expect(v).toBe(2);
  expect(g(v)).toBe(true);
});
it('b', () => {
  const v = f('2');
  expect(v).toBe(3);
  expect(g(v)).toBe(true);
});
it('c', () => {
  const v = f(3);
  expect(v).toBe(4);
  expect(g(v)).toBe(true);
});
```

```expect
(none)
```

### case: skipped and each tests are left out

```js
import { expect, it } from 'vitest';

it.skip('a', () => {
  const v = f(1);
  expect(v).toBe(2);
  expect(g(v)).toBe(true);
});
it.skip('b', () => {
  const v = f(2);
  expect(v).toBe(3);
  expect(g(v)).toBe(true);
});
it.skip('c', () => {
  const v = f(3);
  expect(v).toBe(4);
  expect(g(v)).toBe(true);
});
```

```expect
(none)
```

### case: a title that is not a plain string

```js
import { expect, it } from 'vitest';

it(`a ${n}`, () => {
  const v = f(1);
  expect(v).toBe(2);
  expect(g(v)).toBe(true);
});
it(`b ${n}`, () => {
  const v = f(2);
  expect(v).toBe(3);
  expect(g(v)).toBe(true);
});
it(`c ${n}`, () => {
  const v = f(3);
  expect(v).toBe(4);
  expect(g(v)).toBe(true);
});
```

```expect
(none)
```

### case: tests in a describe block

```js
import { describe, expect, it } from 'vitest';

describe('double', () => {
  it('a', () => {
    const v = f(1);
    expect(v).toBe(2);
    expect(g(v)).toBe(true);
  });
  it('b', () => {
    const v = f(2);
    expect(v).toBe(3);
    expect(g(v)).toBe(true);
  });
  it('c', () => {
    const v = f(3);
    expect(v).toBe(4);
    expect(g(v)).toBe(true);
  });
});
```

```expect
4:6 js/parameterized-tests
```

### case: tests spread over two blocks are not grouped

```js
import { describe, expect, it } from 'vitest';

describe('one', () => {
  it('a', () => {
    const v = f(1);
    expect(v).toBe(2);
    expect(g(v)).toBe(true);
  });
  it('b', () => {
    const v = f(2);
    expect(v).toBe(3);
    expect(g(v)).toBe(true);
  });
});
describe('two', () => {
  it('c', () => {
    const v = f(3);
    expect(v).toBe(4);
    expect(g(v)).toBe(true);
  });
});
```

```expect
(none)
```

### case: Playwright tests

```js
import { expect, test } from '@playwright/test';

test('a', async ({ page }) => {
  await page.goto('/one');
  await expect(page).toHaveTitle('One');
  await expect(page.locator('h1')).toBeVisible();
});
test('b', async ({ page }) => {
  await page.goto('/two');
  await expect(page).toHaveTitle('Two');
  await expect(page.locator('h1')).toBeVisible();
});
test('c', async ({ page }) => {
  await page.goto('/three');
  await expect(page).toHaveTitle('Three');
  await expect(page.locator('h1')).toBeVisible();
});
```

```expect
3:6 js/parameterized-tests
```
