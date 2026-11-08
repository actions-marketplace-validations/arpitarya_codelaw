# js/no-unused-collection — cases

### case: read through length

```js
function g() {
  const a = [1];
  a.push(2);
  log(a.length);
}
```

```expect
(none)
```

### case: assigned later

```js
function g() {
  let acc;
  acc = [];
  acc[0] = 2;
}
```

```expect
2:7 js/no-unused-collection
```
