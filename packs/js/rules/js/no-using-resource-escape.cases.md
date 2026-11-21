# js/no-using-resource-escape — cases

### case: the resource is returned

```js
function openResource() {
  using resource = acquire();
  return resource;
}
```

```expect
3:10 js/no-using-resource-escape
```

### case: a function that captures it is returned

```js
function createReader() {
  using resource = acquire();
  return () => resource.read();
}
```

```expect
3:10 js/no-using-resource-escape
```

### case: the result is returned

```js
function readResource() {
  using resource = acquire();
  return resource.read();
}
```

```expect
(none)
```

### case: the returned function owns its resource

```js
function createReader() {
  return () => {
    using resource = acquire();
    return resource.read();
  };
}
export async function query() {
  await using connection = await connect();
  return await connection.query();
}
```

```expect
(none)
```

### case: inside an object, an array and a conditional

```js
function f(flag) {
  using a = acquire();
  return { a, list: [a], pick: flag ? a : null, read() { return a.read(); } };
}
```

```expect
3:12 js/no-using-resource-escape
3:22 js/no-using-resource-escape
3:39 js/no-using-resource-escape
3:49 js/no-using-resource-escape
```

### case: a local function that captures it

```js
function f() {
  using a = acquire();
  const read = () => a.read();
  return read;
}
```

```expect
4:10 js/no-using-resource-escape
```

### case: a parameter shadows the resource

```js
function f() {
  using a = acquire();
  return (a) => a.read();
}
```

```expect
(none)
```

### case: exports

```js
using resource = acquire();
export { resource };
```

```expect
2:10 js/no-using-resource-escape
```

### case: a nested helper stays inside the scope

```js
function f() {
  using a = acquire();
  const helper = () => a.read();
  return helper();
}
```

```expect
(none)
```
