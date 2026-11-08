# js/no-canvas-to-image — cases

### case: pixels read and written back

```js
ctx2.putImageData(ctx1.getImageData(0, 0, w, h), 0, 0);
```

```expect
1:19 js/no-canvas-to-image
```

### case: the pixels are changed in between

```js
const d = c.getImageData(0, 0, 1, 1);
d.data[0] = 1;
c.putImageData(d, 0, 0);
```

```expect
(none)
```

### case: not a canvas

```js
ctx.drawImage(await loadImage(url.toDataURL()), 0, 0);
```

```expect
(none)
```
