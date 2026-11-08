# js/no-unmodified-loop-condition — cases

### case: one side of a comparison changes

```js
let i = 0;
const n = 3;
while (i < n) {
  i++;
}
```

```expect
(none)
```

### case: neither side changes

```js
let i = 0;
const n = 3;
while (i < n) {
  step();
}
```

```expect
3:8 js/no-unmodified-loop-condition
3:12 js/no-unmodified-loop-condition
```

### case: a call makes it dynamic

```js
let i = 0;
while (ready(i)) {
  step();
}
```

```expect
(none)
```

### case: changed by a called function

```js
let done = false;
function finish() {
  done = true;
}
while (!done) {
  finish();
}
```

```expect
(none)
```
