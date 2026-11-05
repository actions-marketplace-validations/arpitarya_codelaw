# js/no-throw-literal — cases

### case: a variable may hold an Error

```js
throw err;
```

```expect

```

### case: a number is stringified

```js
throw 404;
```

```expect
1:1 js/no-throw-literal
```

```fixed
throw new Error(String(404));
```
