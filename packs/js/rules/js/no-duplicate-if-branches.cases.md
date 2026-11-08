# js/no-duplicate-if-branches — cases

### case: an else-if that ends in else keeps it

```js
if (a) {
  go();
} else if (b) {
  go();
} else {
  stop();
}
```

```expect
1:1 js/no-duplicate-if-branches
```

```fixed
if (a || b) {
  go();
} else {
  stop();
}
```

### case: different bodies stay

```js
if (a) {
  go();
} else if (b) {
  stop();
}
```

```expect
(none)
```
