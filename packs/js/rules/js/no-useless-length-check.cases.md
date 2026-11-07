# js/no-useless-length-check — cases

### case: the check that changes the result stays

```js
const ok = items.length === 0 || items.some(isValid);
```

```expect
(none)
```
