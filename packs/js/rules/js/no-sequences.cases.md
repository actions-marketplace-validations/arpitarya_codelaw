# js/no-sequences — cases

### case: a for header is allowed

```js
for (i = 0, j = 9; i < j; i++, j--) step(i, j);
```

```expect

```

### case: parentheses are allowed

```js
const x = (a(), b());
```

```expect

```
