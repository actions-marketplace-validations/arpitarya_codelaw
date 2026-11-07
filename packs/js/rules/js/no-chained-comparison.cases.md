### case: a chain in a declaration is split

```js
const inRange = 0 <= x < 10;
```

```expect
1:17 js/no-chained-comparison
```

```fixed
const inRange = 0 <= x && x < 10;
```

### case: a chain under a lower-precedence operator is rewritten by hand

```js
const same = flag === 0 <= x < 10;
```

```expect
1:23 js/no-chained-comparison
```
