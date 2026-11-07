# js/no-unsafe-buffer-conversion — cases

### case: Buffer.from

```js
const copy = Buffer.from(bytes.buffer);
```

```expect
1:14 js/no-unsafe-buffer-conversion
```

```fixed
const copy = Buffer.from(bytes.buffer, bytes.byteOffset, bytes.byteLength);
```

### case: a wider element type

```js
const words = new Uint32Array(bytes.buffer);
```

```expect
1:15 js/no-unsafe-buffer-conversion
```

```fixed
const words = new Uint32Array(bytes.buffer, bytes.byteOffset, bytes.byteLength / Uint32Array.BYTES_PER_ELEMENT);
```

### case: slice()

```js
const raw = bytes.buffer.slice();
```

```expect
1:13 js/no-unsafe-buffer-conversion
```

```fixed
const raw = bytes.buffer.slice(bytes.byteOffset, bytes.byteOffset + bytes.byteLength);
```

### case: explicit offsets stay

```js
const view = new Uint8Array(b.buffer, b.byteOffset, b.byteLength);
```

```expect
(none)
```
