# js/prefer-block-statement-over-iife — cases

### case: an async arrow stays

```js
(async () => {
  await run();
})();
```

```expect
(none)
```

### case: an arrow with parameters stays

```js
((x) => {
  use(x);
})(1);
```

```expect
(none)
```

### case: a return keeps the function

```js
(() => {
  if (done) return;
  run();
})();
```

```expect
(none)
```
