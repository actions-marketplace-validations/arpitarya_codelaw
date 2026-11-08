# js/no-repeated-call-wrappers — cases

### case: one call is bare

```js
const g = (s) => s.split(",");
new Set(g(a));
g(b);
```

```expect
(none)
```

### case: Number wrapper

```js
function n(o) {
  return o.c;
}
Number(n(x));
Number(n(y));
```

```expect
1:10 js/no-repeated-call-wrappers
```

### case: exported

```js
export const g = (s) => s;
new Set(g(a));
new Set(g(b));
```

```expect
(none)
```
