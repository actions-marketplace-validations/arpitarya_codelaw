# js/prefer-dispose — cases

### case: one resource

```js
const file = open();
try {
  read(file);
} finally {
  file.close();
}
```

```expect
2:1 js/prefer-dispose
```

### case: two resources, disposed in reverse

```js
const reader = openReader();
const writer = openWriter();
try {
  pipe(reader, writer);
} finally {
  writer.close();
  reader.close();
}
```

```expect
3:1 js/prefer-dispose
```

### case: an awaited disposal

```js
async function run() {
  const connection = await connect();
  try {
    await query(connection);
  } finally {
    await connection.close();
  }
}
```

```expect
3:3 js/prefer-dispose
```

### case: disposed in the wrong order

```js
const reader = openReader();
const writer = openWriter();
try {
  pipe(reader, writer);
} finally {
  reader.close();
  writer.close();
}
```

```expect
(none)
```

### case: a catch clause

```js
const file = open();
try {
  read(file);
} catch (e) {
  log(e);
} finally {
  file.close();
}
```

```expect
(none)
```

### case: the finally does more than dispose

```js
const file = open();
try {
  read(file);
} finally {
  log("done");
  file.close();
}
```

```expect
(none)
```

### case: the resource is read after the try

```js
const file = open();
try {
  read(file);
} finally {
  file.close();
}
report(file);
```

```expect
(none)
```

### case: a resource that is not declared just before

```js
function f(file) {
  try {
    read(file);
  } finally {
    file.close();
  }
}
```

```expect
(none)
```
