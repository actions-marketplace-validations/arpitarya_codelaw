# js/import-no-anonymous-default-export — cases

### case: named declarations, identifiers and calls are fine

```js
const x = 1;
export default x;
```

```expect

```

### case: object literal

```js
export default { a: 1 };
```

```expect
1:1 js/import-no-anonymous-default-export
```

### case: a call is allowed

```js
export default connect(App);
```

```expect

```
