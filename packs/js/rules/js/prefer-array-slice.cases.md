# js/prefer-array-slice — cases

### case: at()

```js
const head = list.splice(index).at(0);
```

```expect
1:14 js/prefer-array-slice
```

```fixed
const head = list.slice(index).at(0);
```

### case: a delete count stays

```js
const head = list.splice(index, count)[0];
```

```expect
(none)
```

### case: a bare splice stays

```js
list.splice(index);
```

```expect
(none)
```
