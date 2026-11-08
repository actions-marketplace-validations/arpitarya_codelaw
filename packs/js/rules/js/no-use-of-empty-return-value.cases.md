# js/no-use-of-empty-return-value — cases

### case: right of ||

```js
function log(m) {
  out.write(m);
}
ok || log("x");
```

```expect
(none)
```

### case: tested

```js
const step = () => {
  tick();
};
if (step()) go();
```

```expect
4:5 js/no-use-of-empty-return-value
```
