# js/no-declarations-before-early-exit — cases

### case: used by the guard

```js
function f(a) {
  const n = 1;
  if (!n) return;
  g(n);
}
```

```expect
(none)
```

### case: a computed value is reported, not fixed

```js
function f(a) {
  const r = calc();
  if (!a) {
    throw e;
  }
  g(r);
}
```

```expect
2:9 js/no-declarations-before-early-exit
```

### case: awaited

```js
async function f(a) {
  const v = await p;
  if (!a) return;
  g(v);
}
```

```expect
(none)
```
