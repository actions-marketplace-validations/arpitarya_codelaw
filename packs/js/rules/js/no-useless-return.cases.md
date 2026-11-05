# js/no-useless-return — cases

### case: an early return is needed

```js
function f() {
  if (a) return;
  run();
}
```

```expect

```
