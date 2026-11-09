# js/no-nonstandard-builtin-properties — cases

### case: a typo is renamed

```js
const pairs = Object.entires(source);
```

```expect
1:22 js/no-nonstandard-builtin-properties
```

```fixed
const pairs = Object.entries(source);
```

### case: an unknown constant

```js
const x = Math.NON_EXISTS_PROPERTY;
```

```expect
1:16 js/no-nonstandard-builtin-properties
```

### case: an unknown static method

```js
Promise.nonExistsMethod();
```

```expect
1:9 js/no-nonstandard-builtin-properties
```

### case: a prototype extension

```js
Array.prototype.customMethod = function () {};
```

```expect
1:17 js/no-nonstandard-builtin-properties
```

### case: a string literal

```js
'text'.foo();
```

```expect
1:8 js/no-nonstandard-builtin-properties
```

### case: a value called

```js
const a = Math.PI();
const b = 'text'.length();
```

```expect
1:16 js/no-nonstandard-builtin-properties
```

### case: a static built with new

```js
const p = new Math.PI();
```

```expect
1:20 js/no-nonstandard-builtin-properties
```

### case: standard members

```js
Object.entries(o);
Math.max(1, 2);
Promise.try(f);
Array.prototype.map.call(list, f);
'text'.padStart(3);
Number.MAX_SAFE_INTEGER;
Object.prototype.hasOwnProperty.call(o, 'k');
```

```expect
(none)
```

### case: a shadowed global

```js
const Object = { custom: 1 };
Object.custom;
```

```expect
(none)
```

### case: globalThis

```js
globalThis.Math.nope();
```

```expect
1:17 js/no-nonstandard-builtin-properties
```

### case: a namespace that is not listed

```js
Intl.NumberFormat.something();
URL.nope();
```

```expect
(none)
```
