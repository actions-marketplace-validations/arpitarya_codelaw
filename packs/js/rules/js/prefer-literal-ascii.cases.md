# js/prefer-literal-ascii — cases

### case: the quote stays escaped

```js
const q = "\x22";
```

```expect
(none)
```

### case: a template dollar stays

```js
const t = `\u0024\u007Bvalue}`;
```

```expect
(none)
```
