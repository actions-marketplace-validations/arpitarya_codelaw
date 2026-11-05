# js/no-nested-ternary — cases

### case: a concise arrow body gets a block

```js
const size = (n) => n > 100 ? 'large' : n > 10 ? 'medium' : 'small';
```

```expect
1:21 js/no-nested-ternary
```

```fixed
const size = (n) => { if (n > 100) return 'large'; if (n > 10) return 'medium'; return 'small'; };
```

### case: a declaration carries no edit

```js
const size = n > 100 ? 'large' : n > 10 ? 'medium' : 'small';
```

```expect
1:14 js/no-nested-ternary
```
