# js/prefer-aggregate-error — cases

### case: a non-zero check

```js
if (validationErrors.length !== 0) {
  throw new Error("Invalid");
}
```

```expect
2:3 js/prefer-aggregate-error
```

```fixed
if (validationErrors.length !== 0) {
  throw new AggregateError(validationErrors, "Invalid");
}
```

### case: another collection stays

```js
if (items.length > 0) {
  throw new Error("Not empty");
}
```

```expect
(none)
```
