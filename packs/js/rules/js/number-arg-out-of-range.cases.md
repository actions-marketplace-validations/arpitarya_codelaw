# js/number-arg-out-of-range — cases

### case: in-range values

```js
a.toString(2); a.toString(36); a.toFixed(0); a.toFixed(100); a.toPrecision(1); a.toString();
```

```expect

```

### case: out-of-range values

```js
a.toFixed(-1);
a.toPrecision(0);
a.toString(37);
```

```expect
1:1 js/number-arg-out-of-range
2:1 js/number-arg-out-of-range
3:1 js/number-arg-out-of-range
```
