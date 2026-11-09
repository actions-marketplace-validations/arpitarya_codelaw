# js/no-object-methods-with-collections — cases

### case: entries of a Set

```js
const set = new Set(['a']);
Object.entries(set);
```

```expect
2:8 js/no-object-methods-with-collections
```

```fixed
const set = new Set(['a']);
Array.from(set.entries());
```

### case: values of a Map

```js
const map = new Map();
const v = Object.values(map);
```

```expect
2:18 js/no-object-methods-with-collections
```

```fixed
const map = new Map();
const v = Array.from(map.values());
```

### case: a conditional receiver is parenthesized only when needed

```js
const a = new Map();
const b = new Map();
Object.keys(flag ? a : b);
```

```expect
3:8 js/no-object-methods-with-collections
```

```fixed
const a = new Map();
const b = new Map();
Array.from((flag ? a : b).keys());
```

### case: a WeakMap is not reported

```js
const map = new WeakMap();
Object.keys(map);
```

```expect
(none)
```

### case: a plain object

```js
const o = {};
Object.keys(o);
```

```expect
(none)
```

### case: Map and Set mixed

```js
const x = flag ? new Map() : new Set();
Object.keys(x);
```

```expect
(none)
```

### case: a shadowed Object

```js
const Object = {};
const map = new Map();
Object.keys(map);
```

```expect
(none)
```

### case: two arguments

```js
const map = new Map();
Object.keys(map, 1);
```

```expect
(none)
```

### case: a comment blocks the suggestion

```js
const map = new Map();
Object.keys(/* c */ map);
```

```expect
2:8 js/no-object-methods-with-collections
```
