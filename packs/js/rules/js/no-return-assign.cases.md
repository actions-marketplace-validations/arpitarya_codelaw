# js/no-return-assign — cases

### case: parentheses mark it intended

```js
function f() {
  return (a = b);
}
```

```expect

```

### case: a concise arrow body gets a block

```js
const set = (v) => value = v;
```

```expect
1:20 js/no-return-assign
```

```fixed
const set = (v) => { value = v; return value; };
```

### case: a member target is outside the auto subset

```js
function set(v) {
  return this.value = v;
}
```

```expect
2:3 js/no-return-assign
```
