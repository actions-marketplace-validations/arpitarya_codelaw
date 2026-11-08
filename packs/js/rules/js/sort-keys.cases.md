# js/sort-keys — cases

### case: a spread starts a new run

```js
const o = { b: 1, ...rest, a: 2 };
```

```expect
(none)
```

### case: a call value has no fix

```js
const o = { b: f(), a: g() };
```

```expect
1:21 js/sort-keys
```
