# js/no-incorrect-query-selector — cases

### case: at(0)

```js
const row = table.querySelectorAll("tr").at(0);
```

```expect
1:13 js/no-incorrect-query-selector
```

```fixed
const row = table.querySelector("tr");
```

### case: another index stays

```js
const row = table.querySelectorAll("tr")[1];
```

```expect
(none)
```
