# js/no-duplicate-string — cases

### case: object keys don't count

```js
const a = { "content-type-x": 1 };
const b = { "content-type-x": 2 };
const c = { "content-type-x": 3 };
```

```expect
(none)
```
