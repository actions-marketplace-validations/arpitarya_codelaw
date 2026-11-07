# js/prefer-simplified-conditions — cases

### case: two negated operands

```js
if (!(!a && !b)) {}
```

```expect
1:5 js/prefer-simplified-conditions
```

```fixed
if (a || b) {}
```

### case: outside a condition it stays

```js
const value = !(!a && !b);
```

```expect
(none)
```

### case: a plain negated group stays

```js
if (!(a && b)) {}
```

```expect
(none)
```
