# js/no-immediate-mutation — cases

### case: uses itself

```js
const a = [1];
a.push(a.length);
```

```expect
(none)
```

### case: not the next statement

```js
const a = [1];
log(a);
a.push(2);
```

```expect
(none)
```

### case: an object property

```js
const o = { a: 1 };
o.b = 2;
```

```expect
2:1 js/no-immediate-mutation
```

```fixed
const o = { a: 1, b: 2 };
```

### case: a map entry

```js
const m = new Map();
m.set("k", 1);
```

```expect
2:1 js/no-immediate-mutation
```

```fixed
const m = new Map([["k", 1]]);
```

### case: let then assign

```js
let s;
s = "x";
```

```expect
2:1 js/no-immediate-mutation
```

```fixed
let s = "x";
```
