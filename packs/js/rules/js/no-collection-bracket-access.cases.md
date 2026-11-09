# js/no-collection-bracket-access — cases

### case: a read from a Map

```js
const map = new Map();
const value = map['foo'];
```

```expect
2:15 js/no-collection-bracket-access
```

### case: a delete is rewritten

```js
const map = new Map();
delete map['foo'];
```

```expect
2:8 js/no-collection-bracket-access
```

```fixed
const map = new Map();
map.delete('foo');
```

### case: a Set read has no suggestion

```js
const set = new Set();
if (set['foo']) {}
```

```expect
2:5 js/no-collection-bracket-access
```

### case: a real member is allowed

```js
const map = new Map();
const n = map['size'];
const it = map[Symbol.iterator];
```

```expect
(none)
```

### case: a computed key

```js
const map = new Map();
map[key] = 1;
```

```expect
2:1 js/no-collection-bracket-access
```

```fixed
const map = new Map();
map.set(key, 1);
```

### case: an annotated parameter

```ts
function f(m: ReadonlyMap<string, number>) {
  return m['a'];
}
```

```expect
2:10 js/no-collection-bracket-access
```

### case: a user's own Map type is not the built-in

```ts
interface Map<K, V> { [key: string]: V }
const m: Map<string, number> = make();
m['a'];
```

```expect
(none)
```

### case: an array is not a collection

```js
const list = [1, 2];
list[0] = 3;
```

```expect
(none)
```

### case: an unknown value

```js
const map = make();
map['foo'] = 1;
```

```expect
(none)
```

### case: a shadowed Map

```js
class Map {}
const map = new Map();
map['foo'] = 1;
```

```expect
(none)
```

### case: a WeakMap write

```js
const wm = new WeakMap();
wm[key] = 1;
```

```expect
2:1 js/no-collection-bracket-access
```

```fixed
const wm = new WeakMap();
wm.set(key, 1);
```

### case: an alias through const

```js
const a = new Map();
const b = a;
b['x'];
```

```expect
3:1 js/no-collection-bracket-access
```
