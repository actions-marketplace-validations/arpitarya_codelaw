# js/prefer-map-from-entries — cases

### case: reads, sets and deletes

```js
const lookup = Object.fromEntries(Object.entries(source));
lookup.a = 1;
delete lookup.b;
use(lookup.c);
```

```expect
1:16 js/prefer-map-from-entries
```

```fixed
const lookup = new Map(Object.entries(source));
lookup.set('a', 1);
lookup.delete('b');
use(lookup.get('c'));
```

### case: a quoted key and keys()

```js
const t = Object.fromEntries(Object.entries(s));
use(t['a-b']);
const ks = Object.keys(t);
```

```expect
1:11 js/prefer-map-from-entries
```

```fixed
const t = new Map(Object.entries(s));
use(t.get('a-b'));
const ks = [...t.keys()];
```

### case: a literal list of pairs

```js
const t = Object.fromEntries([['a', 1], ['b', 2]]);
use(t.a);
```

```expect
1:11 js/prefer-map-from-entries
```

```fixed
const t = new Map([['a', 1], ['b', 2]]);
use(t.get('a'));
```

### case: passed along

```js
const t = Object.fromEntries(Object.entries(s));
foo(t);
```

```expect
(none)
```

### case: unknown entries

```js
const t = Object.fromEntries(pairs);
use(t.a);
```

```expect
(none)
```

### case: an index-like key

```js
const t = Object.fromEntries([['0', 1]]);
use(t[0]);
```

```expect
(none)
```

### case: a method call on it

```js
const t = Object.fromEntries(Object.entries(s));
t.run();
```

```expect
(none)
```

### case: no uses

```js
const t = Object.fromEntries(Object.entries(s));
```

```expect
(none)
```

### case: an exported binding

```js
export const t = Object.fromEntries(Object.entries(s));
use(t.a);
```

```expect
(none)
```

### case: a shadowed Map

```js
class Map {}
const t = Object.fromEntries(Object.entries(s));
use(t.a);
```

```expect
(none)
```

### case: an inherited name

```js
const t = Object.fromEntries(Object.entries(s));
use(t.toString);
```

```expect
(none)
```
