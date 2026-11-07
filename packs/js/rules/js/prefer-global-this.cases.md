# js/prefer-global-this — cases

### case: a local named global stays

```js
const global = {};
global.x = 1;
```

```expect
(none)
```
