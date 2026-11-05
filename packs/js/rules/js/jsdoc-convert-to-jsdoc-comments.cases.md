# js/jsdoc-convert-to-jsdoc-comments — cases

### case: a directive above a function

```js
// eslint-disable-next-line no-unused-vars
function add(a, b) {
  return a + b;
}
```

```expect

```

### case: a comment above an exported arrow function

```js
// Doubles it.
export const double = (n) => n * 2;
```

```expect
1:1 js/jsdoc-convert-to-jsdoc-comments
```

### case: a comment above a plain variable

```js
// the count
let count = 0;
```

```expect

```
