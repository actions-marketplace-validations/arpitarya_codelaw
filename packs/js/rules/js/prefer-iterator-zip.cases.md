# js/prefer-iterator-zip — cases

### case: the && bound

```js
for (let i = 0; i < a.length && i < b.length; i++) {
  use(a[i], b[i]);
}
```

```expect
1:1 js/prefer-iterator-zip
```

```fixed
for (const [element, element_] of Iterator.zip([a, b])) {
  use(element, element_);
}
```

### case: the bound reversed and the step spelled out

```js
for (let i = 0; Math.min(a.length, b.length) > i; i += 1) {
  use(a[i], b[i]);
}
```

```expect
1:1 js/prefer-iterator-zip
```

### case: three arrays

```js
for (let i = 0; i < Math.min(xs.length, ys.length, zs.length); i = i + 1) {
  draw(xs[i], ys[i], zs[i]);
}
```

```expect
1:1 js/prefer-iterator-zip
```

```fixed
for (const [x, y, z] of Iterator.zip([xs, ys, zs])) {
  draw(x, y, z);
}
```

### case: a bound cached in the head

```js
for (let i = 0, n = Math.min(a.length, b.length); i < n; i++) {
  use(a[i], b[i]);
}
```

```expect
1:1 js/prefer-iterator-zip
```

### case: a bound cached before the loop is removed with it

```js
const size = Math.min(names.length, ages.length);
for (let i = 0; i < size; i++) {
  show(names[i], ages[i]);
}
```

```expect
2:1 js/prefer-iterator-zip
```

```fixed
for (const [name, age] of Iterator.zip([names, ages])) {
  show(name, age);
}
```

### case: a cached bound read again

```js
const size = Math.min(a.length, b.length);
for (let i = 0; i < size; i++) {
  use(a[i], b[i]);
}
log(size);
```

```expect
(none)
```

### case: one array only

```js
for (let i = 0; i < a.length; i++) {
  use(a[i]);
}
```

```expect
(none)
```

### case: the index is used for something else

```js
for (let i = 0; i < Math.min(a.length, b.length); i++) {
  use(a[i], b[i], i);
}
```

```expect
(none)
```

### case: an array is read at another place

```js
for (let i = 0; i < Math.min(a.length, b.length); i++) {
  use(a[i], b[i + 1]);
}
```

```expect
(none)
```

### case: an element is written

```js
for (let i = 0; i < Math.min(a.length, b.length); i++) {
  a[i] = b[i];
}
```

```expect
(none)
```

### case: an element is read inside a callback

```js
for (let i = 0; i < Math.min(a.length, b.length); i++) {
  run(() => use(a[i], b[i]));
}
```

```expect
(none)
```

### case: an array that is never read

```js
for (let i = 0; i < Math.min(a.length, b.length); i++) {
  use(a[i]);
}
```

```expect
(none)
```

### case: the same array twice

```js
for (let i = 0; i < Math.min(a.length, a.length); i++) {
  use(a[i]);
}
```

```expect
(none)
```

### case: a string is not zipped

```js
const a = 'abc';
for (let i = 0; i < Math.min(a.length, b.length); i++) {
  use(a[i], b[i]);
}
```

```expect
(none)
```

### case: a comment inside the head leaves no edit

```js
for (let i = 0; i < Math.min(a.length, b.length) /* both */; i++) {
  use(a[i], b[i]);
}
```

```expect
1:1 js/prefer-iterator-zip
```

### case: a name that is taken

```js
const a_ = 1;
for (let i = 0; i < Math.min(a.length, b.length); i++) {
  use(a[i], b[i], a_);
}
```

```expect
2:1 js/prefer-iterator-zip
```

### case: another function's name is not a conflict

```js
function other(element) {}
for (let i = 0; i < Math.min(items.length, tags.length); i++) {
  use(items[i], tags[i]);
}
```

```expect
2:1 js/prefer-iterator-zip
```

```fixed
function other(element) {}
for (const [item, tag] of Iterator.zip([items, tags])) {
  use(item, tag);
}
```
