# js/csrf — cases

### case: safe methods only stay

```js
const protect = csrf({ ignoreMethods: ["GET", "HEAD", "OPTIONS"] });
```

```expect
(none)
```
