# js/no-invalid-intl-options — cases

### case: a date option

```js
const fmt = new Intl.DateTimeFormat("en", { timezone: "UTC" });
```

```expect
1:45 js/no-invalid-intl-options
```

```fixed
const fmt = new Intl.DateTimeFormat("en", { timeZone: "UTC" });
```

### case: valid options stay

```js
const fmt = new Intl.NumberFormat("en", { style: "currency", currency: "EUR" });
```

```expect
(none)
```
