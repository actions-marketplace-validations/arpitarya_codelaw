# js/no-misrefactored-shorthand-assign — cases

### case: the repeated target on the left of the product

```js
size *= step * size;
```

```expect
1:1 js/no-misrefactored-shorthand-assign
```

```fixed
size *= step;
```
