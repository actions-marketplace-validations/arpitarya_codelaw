# js/no-useless-iterator-to-array — cases

### case: Array.from

```js
const a = Array.from(it.toArray());
```

```expect
1:25 js/no-useless-iterator-to-array
```

```fixed
const a = Array.from(it);
```

### case: a spread

```js
const all = [...it.toArray()];
run(...it.toArray());
```

```expect
1:20 js/no-useless-iterator-to-array
2:11 js/no-useless-iterator-to-array
```

```fixed
const all = [...it];
run(...it);
```

### case: for of is a suggestion

```js
for (const x of it.toArray()) {
  use(x);
}
```

```expect
1:20 js/no-useless-iterator-to-array
```

```fixed
for (const x of it) {
  use(x);
}
```

### case: yield star

```js
function* g() {
  yield* it.toArray();
}
```

```expect
2:13 js/no-useless-iterator-to-array
```

```fixed
function* g() {
  yield* it;
}
```

### case: Promise.all

```js
await Promise.all(it.toArray());
```

```expect
1:22 js/no-useless-iterator-to-array
```

```fixed
await Promise.all(it);
```

### case: an iterator method

```js
it.toArray().every((x) => x > 0);
```

```expect
1:4 js/no-useless-iterator-to-array
```

```fixed
it.every((x) => x > 0);
```

### case: a callback that can read the array

```js
it.toArray().some((x, i, all) => all.length > i);
```

```expect
(none)
```

### case: a named callback

```js
it.toArray().every(check);
```

```expect
(none)
```

### case: filter is not reported

```js
it.toArray().filter((x) => x);
```

```expect
(none)
```

### case: a class with its own toArray

```js
class Vec {
  toArray() {
    return [1];
  }
}
const v = new Vec();
new Set(v.toArray());
```

```expect
(none)
```

### case: a literal receiver

```js
new Set([1].toArray());
```

```expect
(none)
```

### case: a second argument is a suggestion

```js
Array.from(it.toArray(), (x) => x);
```

```expect
1:15 js/no-useless-iterator-to-array
```

```fixed
Array.from(it, (x) => x);
```

### case: a shadowed Set

```js
class Set {}
new Set(it.toArray());
```

```expect
(none)
```
