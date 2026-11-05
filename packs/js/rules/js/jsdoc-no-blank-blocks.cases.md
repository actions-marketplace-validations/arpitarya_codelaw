# js/jsdoc-no-blank-blocks — cases

### case: a one-line empty block

```js
const a = 1;
/** */
function run() {}
```

```expect
2:1 js/jsdoc-no-blank-blocks
```

```fixed
const a = 1;
function run() {}
```

### case: the empty block comment /**/ is not JSDoc

```js
/**/
function run() {}
```

```expect

```
