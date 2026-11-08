# js/no-unnecessary-default-arguments — cases

### case: a different value

```js
function r(a, b = 1) {}
r(1, 2);
```

```expect
(none)
```

### case: a middle argument is kept

```js
function r(a = 1, b = 2) {}
r(1, 3);
```

```expect
(none)
```

### case: two trailing

```js
const f = (x, y = 0, z = "px") => x;
f(1, 0, "px");
```

```expect
2:6 js/no-unnecessary-default-arguments
```

```fixed
const f = (x, y = 0, z = "px") => x;
f(1);
```
