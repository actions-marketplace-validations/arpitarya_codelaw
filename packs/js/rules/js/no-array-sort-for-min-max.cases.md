### case: a descending comparator reads the largest

```js
const top = scores.sort((a, b) => b - a)[0];
```

```expect
1:13 js/no-array-sort-for-min-max
```

```fixed
const top = Math.max(...scores);
```

### case: another comparator is rewritten by hand

```js
const first = people.sort((a, b) => a.age - b.age)[0];
```

```expect
1:15 js/no-array-sort-for-min-max
```
