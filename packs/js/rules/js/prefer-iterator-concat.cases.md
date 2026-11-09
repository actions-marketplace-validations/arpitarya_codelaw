# js/prefer-iterator-concat — cases

### case: for of is a suggestion

```js
for (const item of [...first, ...second]) {
  use(item);
}
```

```expect
1:20 js/prefer-iterator-concat
```

```fixed
for (const item of Iterator.concat(first, second)) {
  use(item);
}
```

### case: Object.fromEntries

```js
const o = Object.fromEntries([...a, ...b]);
```

```expect
1:30 js/prefer-iterator-concat
```

```fixed
const o = Object.fromEntries(Iterator.concat(a, b));
```

### case: a later argument that reads a member is a suggestion

```js
const s = new Set([...a, ...b.items]);
```

```expect
1:19 js/prefer-iterator-concat
```

```fixed
const s = new Set(Iterator.concat(a, b.items));
```

### case: mixed elements

```js
new Set([first, ...rest]);
```

```expect
(none)
```

### case: a spread of a string

```js
new Set([...'abc', ...'xyz']);
```

```expect
(none)
```

### case: a plain statement

```js
const all = [...a, ...b];
```

```expect
(none)
```

### case: known sets

```js
const a = new Set([1]);
const b = new Set([2]);
const u = new Set([...a, ...b]);
```

```expect
(none)
```

### case: a toArray spread

```js
new Set([...a.toArray(), ...b]);
```

```expect
(none)
```
