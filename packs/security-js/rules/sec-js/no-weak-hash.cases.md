# sec-js/no-weak-hash — cases

### case: sha1 in createHmac

```js
crypto.createHmac("sha1", key).update(d);
```

```expect
1:1 sec-js/no-weak-hash
```

### case: sha256

```js
crypto.createHash("sha256");
```

```expect
(none)
```

### case: crypto.subtle

```js
crypto.subtle.digest("SHA-1", data);
```

```expect
1:1 sec-js/no-weak-hash
```

### case: fix: sha256

```js
crypto.createHash("sha1");
```

```expect
1:1 sec-js/no-weak-hash
```

```fixed
crypto.createHash("sha256");
```

### case: fix: subtle digest name

```js
crypto.subtle.digest("SHA-1", data);
```

```expect
1:1 sec-js/no-weak-hash
```

```fixed
crypto.subtle.digest("SHA-256", data);
```
