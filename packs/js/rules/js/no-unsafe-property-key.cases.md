# js/no-unsafe-property-key — cases

### case: a BigInt index

```js
items[4n] = value;
```

```expect
1:1 js/no-unsafe-property-key
```

### case: an object literal key

```js
const o = { [{}]: 1 };
```

```expect
1:13 js/no-unsafe-property-key
```

### case: a number index stays

```js
items[4] = value;
```

```expect
(none)
```
