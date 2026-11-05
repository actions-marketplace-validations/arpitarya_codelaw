# js/no-boolean-sort-comparator — cases

### case: a descending boolean comparator

```js
items.sort((a, b) => a < b);
```

```expect
1:12 js/no-boolean-sort-comparator
```

```fixed
items.sort((a, b) => b - a);
```
