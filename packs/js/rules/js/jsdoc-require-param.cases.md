# js/jsdoc-require-param — cases

### case: an exported arrow function is checked

```js
/** Doubles. */
export const double = (n) => n * 2;
```

```expect
1:1 js/jsdoc-require-param
```

### case: a method is checked

```js
class A {
  /** Sets the size. */
  resize(w) {}
}
```

```expect
2:3 js/jsdoc-require-param
```

### case: a function without parameters needs none

```js
/** Ticks. */
function tick() {}
```

```expect

```

### case: @inheritdoc is left alone

```js
class B {
  /** @inheritdoc */
  resize(w) {}
}
```

```expect

```

### case: a plain block comment is not JSDoc

```js
/* Adds. */
function add(a, b) {}
```

```expect

```
