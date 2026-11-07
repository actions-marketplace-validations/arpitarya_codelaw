# js/content-length — cases

### case: IncomingForm

```js
const form = new IncomingForm({ keepExtensions: true });
```

```expect
1:14 js/content-length
```

### case: options from elsewhere stay

```js
const form = formidable(options);
```

```expect
(none)
```
