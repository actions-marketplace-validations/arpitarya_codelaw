# js/no-useless-boolean-cast — cases

### case: some

```js
const any = rows.some((row) => Boolean(row.error));
```

```expect
1:32 js/no-useless-boolean-cast
```

```fixed
const any = rows.some((row) => row.error);
```

### case: an optional chain stays

```js
const xs = rows.filter((row) => Boolean(row?.error));
```

```expect
(none)
```

### case: map stays

```js
const flags = rows.map((row) => Boolean(row.error));
```

```expect
(none)
```
