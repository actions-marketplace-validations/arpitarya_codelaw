# js/no-blob-to-file — cases

### case: form data takes the name

```js
const b = new Blob();
const d = new FormData();
const f = new File([b], "x");
d.set("k", f);
```

```expect
3:11 js/no-blob-to-file
```

```fixed
const b = new Blob();
const d = new FormData();
d.set("k", b, "x");
```

### case: used twice

```js
const b = new Blob();
const f = new File([b], "a");
use(f);
use(f);
```

```expect
(none)
```

### case: two parts

```js
const b = new Blob();
const f = new File([b, b], "a");
URL.createObjectURL(f);
```

```expect
(none)
```
