# node/exports-style — cases

### case: a file that replaces module.exports is not auto-fixed or reported

```js
module.exports = {};
exports.a = 1;
```

```expect

```

### case: a local named exports is left alone

```js
function f(exports) {
  exports.a = 1;
}
```

```expect

```
