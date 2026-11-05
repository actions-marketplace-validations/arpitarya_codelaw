# sec-js/no-log-injection — cases

### case: console.log

```js
console.log(req.body.name);
```

```expect
1:1 sec-js/no-log-injection
```

### case: fix: line breaks are stripped

```js
logger.info(req.body.user);
```

```expect
1:1 sec-js/no-log-injection
```

```fixed
logger.info(String(req.body.user).replace(/[\r\n]+/g, " "));
```
