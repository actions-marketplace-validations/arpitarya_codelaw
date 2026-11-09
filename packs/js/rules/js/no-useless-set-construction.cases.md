# js/no-useless-set-construction — cases

### case: the keys of a Map

```js
const a = new Set([1]);
const m = new Map();
a.intersection(new Set(m.keys()));
```

```expect
3:16 js/no-useless-set-construction
```

```fixed
const a = new Set([1]);
const m = new Map();
a.intersection(m);
```

### case: a copy of the receiver

```js
const a = new Set([1]);
const b = new Set([2]);
new Set(a).union(b);
```

```expect
3:1 js/no-useless-set-construction
```

```fixed
const a = new Set([1]);
const b = new Set([2]);
a.union(b);
```

### case: a copy of a result

```js
const a = new Set([1]);
const b = new Set([2]);
const c = new Set(a.union(b));
```

```expect
3:11 js/no-useless-set-construction
```

```fixed
const a = new Set([1]);
const b = new Set([2]);
const c = a.union(b);
```

### case: map values are not keys

```js
const a = new Set([1]);
const m = new Map();
a.union(new Set(m.values()));
```

```expect
(none)
```

### case: a receiver copy with a side effect in the argument

```js
const a = new Set([1]);
const b = new Set([2]);
new Set(a).union((a.clear(), b));
```

```expect
(none)
```

### case: an unknown argument

```js
const a = new Set([1]);
a.union(new Set(list));
```

```expect
(none)
```

### case: an unknown receiver

```js
a.union(new Set(b));
```

```expect
(none)
```

### case: type arguments

```ts
const a = new Set<number>([1]);
const b = new Set<number>([2]);
a.union(new Set<number>(b));
```

```expect
(none)
```

### case: a receiver copy of a known Set argument

```js
const a = new Set([1]);
const b = new Set([2]);
const r = new Set(a).isSubsetOf(b);
```

```expect
3:11 js/no-useless-set-construction
```

```fixed
const a = new Set([1]);
const b = new Set([2]);
const r = a.isSubsetOf(b);
```
