# js/prefer-https — cases

### case: a comment

```js
// Docs: http://developer.mozilla.org/en-US/
```

```expect
1:1 js/prefer-https
```

```fixed
// Docs: https://developer.mozilla.org/en-US/
```

### case: localhost stays

```js
const dev = "http://localhost:3000";
```

```expect
(none)
```

### case: an XML namespace stays

```js
const ns = "http://www.w3.org/2000/svg";
```

```expect
(none)
```

### case: a test domain stays

```js
const url = "http://shop.test/cart";
```

```expect
(none)
```
