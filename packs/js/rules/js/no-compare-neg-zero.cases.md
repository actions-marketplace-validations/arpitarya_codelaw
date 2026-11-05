# js/no-compare-neg-zero — cases

### case: a negated strict comparison

```js
if (-0 !== x) run();
```

```expect
1:5 js/no-compare-neg-zero
```

```fixed
if (0 !== x) run();
```
