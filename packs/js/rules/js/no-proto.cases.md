# js/no-proto — cases

### case: a write becomes setPrototypeOf

```js
obj.__proto__ = base;
```

```expect
1:1 js/no-proto
```

```fixed
Object.setPrototypeOf(obj, base);
```

### case: a bracket read

```js
const p = obj["__proto__"];
```

```expect
1:11 js/no-proto
```

```fixed
const p = Object.getPrototypeOf(obj);
```
