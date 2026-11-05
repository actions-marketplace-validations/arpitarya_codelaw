# js/no-negated-condition — cases

### case: an else-if chain is allowed

```js
if (!a) {
  x();
} else if (b) {
  y();
}
```

```expect

```

### case: no else is allowed

```js
if (!a) x();
```

```expect

```

### case: a compound condition is allowed

```js
if (!a && b) {
  x();
} else {
  y();
}
```

```expect

```

### case: a negated ternary

```js
const v = a !== b ? 1 : 2;
```

```expect
1:11 js/no-negated-condition
```

```fixed
const v = a === b ? 2 : 1;
```

### case: a not-equal test swaps the branches

```js
if (a != b) {
  x();
} else {
  y();
}
```

```expect
1:1 js/no-negated-condition
```

```fixed
if (a == b) {
  y();
} else {
  x();
}
```
