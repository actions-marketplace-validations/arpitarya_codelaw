# js/no-negated-array-predicate — cases

### case: a negated every

```js
const bad = !items.every((item) => isValid(item));
```

```expect
1:13 js/no-negated-array-predicate
```

```fixed
const bad = items.some((item) => !isValid(item));
```

### case: a callback that already negates

```js
if (!items.some((item) => !isValid(item))) {}
```

```expect
1:5 js/no-negated-array-predicate
```

```fixed
if (items.every((item) => isValid(item))) {}
```

### case: a callback with a block body stays

```js
const bad = !items.every((item) => { return ok(item); });
```

```expect
(none)
```

### case: a function reference stays

```js
const bad = !items.every(Boolean);
```

```expect
(none)
```
