# js/prefer-promise-with-resolvers — cases

### case: the executor does work

```js
new Promise((r) => {
  setup();
  r();
});
```

```expect
(none)
```

### case: both resolvers

```js
let ok, no;
const done = new Promise((a, b) => {
  ok = a;
  no = b;
});
```

```expect
2:14 js/prefer-promise-with-resolvers
```

```fixed
const { promise: done, resolve: ok, reject: no } = Promise.withResolvers();
```

### case: an expression body

```js
let res;
const p = new Promise((r) => (res = r));
```

```expect
(none)
```
