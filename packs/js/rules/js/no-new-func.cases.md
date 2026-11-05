# js/no-new-func — cases

### case: a body only, called without new

```js
const one = Function('return 1');
```

```expect
1:13 js/no-new-func
```

```fixed
const one = function () { return 1 };
```

### case: a body with an escape carries no edit

```js
const f = new Function('a', 'return a + "\n"');
```

```expect
1:11 js/no-new-func
```

### case: a computed body carries no edit

```js
const f = new Function('a', body);
```

```expect
1:11 js/no-new-func
```
