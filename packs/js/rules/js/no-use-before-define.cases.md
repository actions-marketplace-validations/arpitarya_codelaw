# js/no-use-before-define — cases

### case: functions allowed

```js options={"functions":false}
run();
function run() {}
```

```expect
(none)
```

### case: own initialiser

```js
const a = a + 1;
```

```expect
1:11 js/no-use-before-define
```

### case: named export above

```js options={"allowNamedExports":true}
export { x };
const x = 1;
```

```expect
(none)
```
