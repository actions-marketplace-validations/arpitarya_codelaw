# js/prefer-array-iterable-methods — cases

### case: the value is unused

```js
const list = [1];
for (const [index] of list.entries()) {
  use(index);
}
```

```expect
2:28 js/prefer-array-iterable-methods
```

```fixed
const list = [1];
for (const index of list.keys()) {
  use(index);
}
```

### case: both are used

```js
const list = [1];
for (const [i, v] of list.entries()) {
  use(i, v);
}
```

```expect
(none)
```

### case: both are unused

```js
const list = [1];
for (const [, ] of list.entries()) {}
```

```expect
(none)
```

### case: a Map is not an array

```js
const map = new Map();
for (const [, v] of map.entries()) {
  use(v);
}
```

```expect
(none)
```

### case: an unknown receiver

```js
for (const [, v] of things.entries()) {
  use(v);
}
```

```expect
(none)
```

### case: a typed array

```ts
function f(xs: string[]) {
  for (const [, x] of xs.entries()) {
    use(x);
  }
}
```

```expect
2:26 js/prefer-array-iterable-methods
```

### case: a default on the kept binding

```js
const list = [1];
for (const [, v = 1] of list.entries()) {
  use(v);
}
```

```expect
(none)
```

### case: for await

```js
const list = [1];
async function f() {
  for await (const [, v] of list.entries()) {
    use(v);
  }
}
```

```expect
(none)
```
