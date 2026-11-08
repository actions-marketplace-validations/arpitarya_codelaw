# js/import-unambiguous — cases

### case: a CommonJS file

```js path=src/a.cjs
module.exports = 1;
```

```expect
(none)
```

### case: empty export

```js
const a = 1;
export {};
```

```expect
(none)
```
