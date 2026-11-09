# js/prefer-iterator-helpers — cases

### case: Array.from and some

```js
const ok = Array.from(map.keys()).some((k) => k.startsWith('a'));
```

```expect
1:12 js/prefer-iterator-helpers
```

```fixed
const ok = map.keys().some((k) => k.startsWith('a'));
```

### case: a slice after toArray

```js
const page = it.toArray().slice(20, 30);
```

```expect
1:27 js/prefer-iterator-helpers
```

```fixed
const page = it.drop(20).take(10).toArray();
```

### case: a slice of a spread

```js
const first = [...text.matchAll(re)].slice(0, 10);
```

```expect
1:38 js/prefer-iterator-helpers
```

```fixed
const first = text.matchAll(re).take(10).toArray();
```

### case: a plain iterable is not known

```js
const hit = [...set].find((v) => v > 1);
```

```expect
(none)
```

### case: copy-only slices

```js
const a = it.toArray().slice(0);
const b = it.toArray().slice();
```

```expect
(none)
```

### case: a callback that reads the array

```js
const hit = [...map.values()].find((v, i, all) => all.length);
```

```expect
(none)
```

### case: a named callback declared here

```js
function pick(v) {
  return v > 1;
}
const hit = [...map.values()].find(pick);
```

```expect
4:13 js/prefer-iterator-helpers
```

```fixed
function pick(v) {
  return v > 1;
}
const hit = map.values().find(pick);
```

### case: a named callback from elsewhere

```js
const hit = [...map.values()].find(pick);
```

```expect
(none)
```

### case: a non-literal bound

```js
const page = it.toArray().slice(start, end);
```

```expect
(none)
```
