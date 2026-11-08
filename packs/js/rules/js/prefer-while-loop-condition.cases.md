# js/prefer-while-loop-condition — cases

### case: a positive condition is negated

```js
for (;;) {
  if (done()) break;
  step();
}
```

```expect
1:1 js/prefer-while-loop-condition
```

```fixed
while (!(done())) {
  step();
}
```
