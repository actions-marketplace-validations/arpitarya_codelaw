# js/prefer-simple-sort-comparator — cases

### case: the if chain

```js
scores.sort((a, b) => {
  if (a > b) {
    return 1;
  }
  if (a < b) {
    return -1;
  }
  return 0;
});
```

```expect
1:1 js/prefer-simple-sort-comparator
```

```fixed
scores.sort((a, b) => a - b);
```

### case: descending

```js
const top = scores.toSorted((a, b) => a > b ? -1 : 1);
```

```expect
1:13 js/prefer-simple-sort-comparator
```

```fixed
const top = scores.toSorted((a, b) => b - a);
```

### case: a property comparison stays

```js
rows.sort((a, b) => a.age > b.age ? 1 : -1);
```

```expect
(none)
```

### case: a random shuffle stays

```js
rows.sort(() => Math.random() > 0.5 ? 1 : -1);
```

```expect
(none)
```
