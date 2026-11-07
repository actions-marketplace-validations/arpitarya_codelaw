# js/prefer-url-can-parse — cases

### case: more work in the try stays

```js
function f(text) {
  try {
    new URL(text);
    log(text);
    return true;
  } catch {
    return false;
  }
}
```

```expect
(none)
```
