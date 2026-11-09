# js/no-unused-iterator-helper — cases

### case: a discarded map is a forEach

```js
items.values().map(save);
```

```expect
1:16 js/no-unused-iterator-helper
```

```fixed
items.values().forEach(save);
```

### case: under void

```js
void items.keys().filter(ok);
```

```expect
1:19 js/no-unused-iterator-helper
```

### case: under await

```js
async function f() {
  await items.entries().flatMap(split);
}
```

```expect
2:25 js/no-unused-iterator-helper
```

### case: a chain

```js
items.values().filter(ok).map(save);
```

```expect
1:27 js/no-unused-iterator-helper
```

```fixed
items.values().filter(ok).forEach(save);
```

### case: the result is used

```js
const out = items.values().map(f);
consume(out);
```

```expect
(none)
```

### case: an array's map

```js
const list = [1];
list.map(save);
```

```expect
(none)
```

### case: an unknown receiver

```js
make().map(save);
```

```expect
(none)
```

### case: a local generator

```js
function* gen() {
  yield 1;
}
gen().take(2);
```

```expect
4:7 js/no-unused-iterator-helper
```

### case: a const alias of an iterator

```js
const it = items.values();
it.drop(1);
```

```expect
2:4 js/no-unused-iterator-helper
```

### case: a TypeScript assertion blocks the suggestion

```ts
(items.values().map(save) as unknown);
```

```expect
1:17 js/no-unused-iterator-helper
```

### case: a typed parameter

```ts
function f(it: IterableIterator<number>) {
  it.map(save);
}
```

```expect
2:6 js/no-unused-iterator-helper
```

```fixed
function f(it: IterableIterator<number>) {
  it.forEach(save);
}
```
