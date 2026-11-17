# js/no-duplicate-map-keys — cases

### case: the same key, not next to each other

```js
const m = new Map([["a", 1], ["b", 2], ["a", 3]]);
```

```expect
1:40 js/no-duplicate-map-keys
```

### case: a computed key twice

```js
const m = new Map([[key(), 1], [key(), 2]]);
```

```expect
(none)
```

### case: a number and its string

```js
const m = new Map([[1, "a"], ["1", "b"]]);
```

```expect
(none)
```
