# js/prefer-iterator-to-array — cases

### case: a certain iterator is fixed

```js
const a = [...Iterator.from(src).map((x) => x * 2)];
```

```expect
1:11 js/prefer-iterator-to-array
```

```fixed
const a = Iterator.from(src).map((x) => x * 2).toArray();
```

### case: values() is a suggestion

```js
const v = [...map.values()];
```

```expect
1:11 js/prefer-iterator-to-array
```

```fixed
const v = map.values().toArray();
```

### case: a collection constructor takes the iterable

```js
const s = new Set([...map.values()]);
```

```expect
(none)
```

### case: for of takes the iterable

```js
for (const x of [...map.keys()]) {}
```

```expect
(none)
```

### case: mixed elements

```js
const v = [first, ...map.values()];
```

```expect
(none)
```

### case: two spreads

```js
const v = [...a.values(), ...b.values()];
```

```expect
(none)
```

### case: a plain iterable

```js
const v = [...set];
```

```expect
(none)
```

### case: matchAll

```js
const m = [...text.matchAll(re)];
```

```expect
1:11 js/prefer-iterator-to-array
```

```fixed
const m = text.matchAll(re).toArray();
```

### case: a comment blocks the fix

```js
const v = [.../* c */ map.values()];
```

```expect
1:11 js/prefer-iterator-to-array
```

### case: a conditional receiver is parenthesized

```js
const v = [...(a ?? b).values()];
```

```expect
1:11 js/prefer-iterator-to-array
```

```fixed
const v = (a ?? b).values().toArray();
```
