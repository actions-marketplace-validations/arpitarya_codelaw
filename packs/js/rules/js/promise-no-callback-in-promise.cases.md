# js/promise-no-callback-in-promise — cases

### case: a call outside the chain is fine

```js
function load(cb) {
  cb(null, 1);
}
```

```expect

```
