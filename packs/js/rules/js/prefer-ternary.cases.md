# js/prefer-ternary — cases

### case: an assignment to the same target

```js
if (wide) {
  width = 100;
} else {
  width = 50;
}
```

```expect
1:1 js/prefer-ternary
```

```fixed
width = wide ? 100 : 50;
```

### case: different targets stay

```js
if (wide) {
  width = 100;
} else {
  height = 50;
}
```

```expect
(none)
```

### case: a branch that is already a ternary stays

```js
if (a) {
  return b ? 1 : 2;
} else {
  return 3;
}
```

```expect
(none)
```

### case: a branch with a second statement stays

```js
if (wide) {
  width = 100;
  height = 10;
} else {
  width = 50;
}
```

```expect
(none)
```
