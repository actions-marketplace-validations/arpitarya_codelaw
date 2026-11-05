# js/no-async-promise-executor — cases

### case: no await, so async is dropped

```js
const p = new Promise(async (resolve) => {
  resolve(1);
});
```

```expect
1:11 js/no-async-promise-executor
```

```fixed
const p = new Promise((resolve) => {
  resolve(1);
});
```

### case: a function executor without await

```js
const p = new Promise(async function (resolve) {
  resolve(1);
});
```

```expect
1:11 js/no-async-promise-executor
```

```fixed
const p = new Promise(function (resolve) { resolve(1); });
```

### case: with await there is no edit

```js
const p = new Promise(async (resolve) => {
  resolve(await load());
});
```

```expect
1:11 js/no-async-promise-executor
```
