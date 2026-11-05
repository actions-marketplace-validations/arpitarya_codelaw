# js/no-useless-catch — cases

### case: with a finally only the catch goes

```js
try {
  a();
} catch (e) {
  throw e;
} finally {
  b();
}
```

```expect
3:3 js/no-useless-catch
```

```fixed
try {
  a();
}  finally {
  b();
}
```
