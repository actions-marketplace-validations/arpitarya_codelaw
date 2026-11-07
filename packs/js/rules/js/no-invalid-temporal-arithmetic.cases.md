# js/no-invalid-temporal-arithmetic — cases

### case: weeks

```js
const earlier = Temporal.Instant.from(stamp).subtract({ weeks: n });
```

```expect
1:17 js/no-invalid-temporal-arithmetic
```

```fixed
const earlier = Temporal.Instant.from(stamp).subtract({ hours: 168 * n });
```

### case: a plain date stays

```js
const later = Temporal.PlainDate.from(day).add({ days: 1 });
```

```expect
(none)
```
