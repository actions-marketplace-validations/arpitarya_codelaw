# js/no-unused-properties — cases

### case: passed on

```js
const c = { a: 1, b: 2 };
use(c);
```

```expect
(none)
```

### case: a method makes this its own

```js
const c = { a: 1, m() {} };
use(c.a);
```

```expect
(none)
```

### case: nested

```js
const s = { x: { y: 1, z: 2 } };
use(s.x.y);
```

```expect
1:24 js/no-unused-properties
```

```fixed
const s = { x: { y: 1 } };
use(s.x.y);
```

### case: destructured

```js
const c = { a: 1, b: 2 };
const { a, b } = c;
use(a, b);
```

```expect
(none)
```
