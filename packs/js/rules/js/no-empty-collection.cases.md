# js/no-empty-collection — cases

### case: pushed to

```js
function f() {
  const q = [];
  q.push(1);
  q.forEach(run);
}
```

```expect
(none)
```

### case: a map read

```js
function g() {
  const m = new Map();
  return m.get("k");
}
```

```expect
3:10 js/no-empty-collection
```
