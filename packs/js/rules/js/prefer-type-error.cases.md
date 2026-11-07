# js/prefer-type-error — cases

### case: a value check is not a type check

```js
if (count < 0) {
  throw new Error("count must not be negative");
}
```

```expect
(none)
```

### case: instanceof without braces

```js
if (!(value instanceof Date)) throw new Error("not a date");
```

```expect
1:37 js/prefer-type-error
```

```fixed
if (!(value instanceof Date)) throw new TypeError("not a date");
```
