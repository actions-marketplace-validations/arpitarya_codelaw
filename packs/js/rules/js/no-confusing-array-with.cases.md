# js/no-confusing-array-with — cases

### case: a negative index

```js
const next = items.with(-1, item);
```

```expect
1:14 js/no-confusing-array-with
```

### case: a computed in-range index stays

```js
const next = items.with(items.length - 1, item);
```

```expect
(none)
```

### case: the length of another array stays

```js
const next = items.with(other.length, item);
```

```expect
(none)
```
