# js/jsdoc-require-jsdoc — cases

### case: an exported function with a block above the export

```js
/** Doubles a number. */
export function double(n) {
  return n * 2;
}
```

```expect

```

### case: a line comment is not JSDoc

```js
// Doubles a number.
export function double(n) {
  return n * 2;
}
```

```expect
2:8 js/jsdoc-require-jsdoc
```

### case: arrow functions are not required by default

```js
const double = (n) => n * 2;
```

```expect

```
