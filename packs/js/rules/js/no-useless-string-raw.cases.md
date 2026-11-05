# js/no-useless-string-raw — cases

### case: a substitution

```js
const p = String.raw`${base}/bin`;
```

```expect
1:11 js/no-useless-string-raw
```

```fixed
const p = `${base}/bin`;
```
