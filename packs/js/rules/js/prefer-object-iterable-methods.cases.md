# js/prefer-object-iterable-methods — cases

### case: keys and values both needed

```js
Object.keys(object).map((key) => foo(object[key], key));
```

```expect
1:8 js/prefer-object-iterable-methods
```

```fixed
Object.entries(object).map(([key, value]) => foo(value, key));
```

### case: values only in a callback

```js
const doubled = Object.keys(object).map((key) => object[key] * 2);
```

```expect
1:24 js/prefer-object-iterable-methods
```

```fixed
const doubled = Object.values(object).map((value) => value * 2);
```

### case: entries with the key unused

```js
Object.entries(object).reduce((result, [key, value]) => result + value, 0);
```

```expect
1:8 js/prefer-object-iterable-methods
```

```fixed
Object.values(object).reduce((result, value) => result + value, 0);
```

### case: entries with the value unused

```js
for (const [key] of Object.entries(object)) {
  foo(key);
}
```

```expect
1:28 js/prefer-object-iterable-methods
```

```fixed
for (const key of Object.keys(object)) {
  foo(key);
}
```

### case: both halves used

```js
for (const [key, value] of Object.entries(object)) {
  foo(key, value);
}
```

```expect
(none)
```

### case: the key is used and the object is not read by it

```js
for (const key of Object.keys(object)) {
  foo(key);
}
```

```expect
(none)
```

### case: the object is written to

```js
for (const key of Object.keys(object)) {
  object[key] = foo(object[key]);
}
```

```expect
(none)
```

### case: a call after the read keeps the report without a fix

```js
for (const key of Object.keys(object)) {
  foo(object[key]);
  bar();
}
```

```expect
1:26 js/prefer-object-iterable-methods
```

### case: a break keeps the report without a fix

```js
for (const key of Object.keys(object)) {
  if (object[key]) break;
}
```

```expect
1:26 js/prefer-object-iterable-methods
```

### case: another object

```js
for (const key of Object.keys(a)) {
  foo(b[key]);
}
```

```expect
(none)
```

### case: Object.keys with forEach is not rewritten

```js
Object.keys(object).forEach((key) => foo(object[key]));
```

```expect
(none)
```
