# js/isolated-functions — cases

### case: builtins are fine

```js
makeSynchronous(() => Math.max(1, 2));
```

```expect
(none)
```

### case: an @isolated comment

```js
/** @isolated */
function f() {
  return this.x;
}
```

```expect
3:10 js/isolated-functions
```

### case: a page.evaluate callback

```js
const sel = "a";
await page.evaluate(() => document.querySelector(sel));
```

```expect
2:50 js/isolated-functions
```
