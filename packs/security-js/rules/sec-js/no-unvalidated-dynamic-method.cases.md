# sec-js/no-unvalidated-dynamic-method — cases

### case: a name bound to the request in the same function

```js
function run(req) {
  const fn = req.params.fn;
  api[fn]();
}
```

```expect
3:3 sec-js/no-unvalidated-dynamic-method
```

### case: a literal key

```js
api["list"]();
```

```expect
(none)
```

### case: a koa query

```js
api[ctx.query.fn]();
```

```expect
1:1 sec-js/no-unvalidated-dynamic-method
```

### case: fix: own-key check keeps this

```js
api[req.query.fn](1);
```

```expect
1:1 sec-js/no-unvalidated-dynamic-method
```

```fixed
(Object.hasOwn(api, req.query.fn) ? api[req.query.fn] : null).call(api, 1);
```
