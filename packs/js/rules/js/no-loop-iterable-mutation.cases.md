# js/no-loop-iterable-mutation — cases

### case: deleting the current item of a set

```js
const s = new Set([1]);
for (const v of s) {
  s.delete(v);
}
```

```expect
(none)
```

### case: adding the current item back

```js
const s = new Set([1]);
for (const v of s) {
  s.add(v);
}
```

```expect
(none)
```

### case: delete then add

```js
const s = new Set([1]);
for (const v of s) {
  s.delete(v);
  s.add(v);
}
```

```expect
4:5 js/no-loop-iterable-mutation
```

### case: map keys

```js
const m = new Map();
for (const [k] of m.entries()) {
  m.delete(k);
}
```

```expect
(none)
```
