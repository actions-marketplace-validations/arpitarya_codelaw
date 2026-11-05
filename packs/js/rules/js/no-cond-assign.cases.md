# js/no-cond-assign — cases

### case: a loop condition is compared

```js
while (a = next()) {
  use(a);
}
```

```expect
1:8 js/no-cond-assign
```

```fixed
while (a === next()) {
  use(a);
}
```
