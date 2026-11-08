# js/no-loop-func — cases

### case: a let never changed after the loop starts

```js
let base = 10;
for (let i = 0; i < 3; i++) {
  run(() => base + i);
}
```

```expect
(none)
```

### case: a let changed in the loop

```js
let total = 0;
for (const x of xs) {
  total += x;
  run(() => total);
}
```

```expect
4:7 js/no-loop-func
```

### case: an IIFE

```js
for (var i = 0; i < 3; i++) {
  (function () {
    use(i);
  })();
}
```

```expect
(none)
```
