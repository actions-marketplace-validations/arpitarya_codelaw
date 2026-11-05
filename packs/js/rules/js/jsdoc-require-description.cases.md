# js/jsdoc-require-description — cases

### case: inheritdoc is exempt

```js
class A {
  /** @inheritdoc */
  run() {}
}
```

```expect

```

### case: a block on a method

```js
class A {
  /** @returns {number} one */
  run() {
    return 1;
  }
}
```

```expect
2:3 js/jsdoc-require-description
```
