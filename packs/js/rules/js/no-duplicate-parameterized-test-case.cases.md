# js/no-duplicate-parameterized-test-case — cases

### case: a repeat on its own line

```js
it.each([
  [1, 2],
  [3, 4],
  [1, 2],
])('adds', (a, b) => {});
```

```expect
4:3 js/no-duplicate-parameterized-test-case
```

```fixed
it.each([
  [1, 2],
  [3, 4],
])('adds', (a, b) => {});
```

### case: an inline repeat

```js
test.each([1, 2, 2, 3])('is', (n) => {});
```

```expect
1:18 js/no-duplicate-parameterized-test-case
```

```fixed
test.each([1, 2, 3])('is', (n) => {});
```

### case: spacing does not matter

```js
describe.each([
  { a: 1 },
  {a:1},
])('d', ({ a }) => {});
```

```expect
3:3 js/no-duplicate-parameterized-test-case
```

```fixed
describe.each([
  { a: 1 },
])('d', ({ a }) => {});
```

### case: two repeats of one row

```js
it.each([1, 1, 1])('is', (n) => {});
```

```expect
1:13 js/no-duplicate-parameterized-test-case
```

```fixed
it.each([1])('is', (n) => {});
```

### case: distinct rows

```js
it.each([[1, 2], [2, 1]])('is', (a, b) => {});
```

```expect
(none)
```

### case: an .only table

```js
it.only.each([1, 1])('is', (n) => {});
```

```expect
1:18 js/no-duplicate-parameterized-test-case
```

```fixed
it.only.each([1])('is', (n) => {});
```

### case: not a test table

```js
other.each([1, 1])('is', (n) => {});
```

```expect
(none)
```

### case: a comment blocks the fix

```js
it.each([
  1,
  1, // again
])('is', (n) => {});
```

```expect
3:3 js/no-duplicate-parameterized-test-case
```
