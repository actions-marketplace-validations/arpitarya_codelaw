# js/no-invalid-argument-count — cases

### case: too many for a local arrow

```js
const sum = (a, b) => a + b;
sum(1, 2, 3);
```

```expect
2:1 js/no-invalid-argument-count
```

### case: too few for a local function is left

```js
function sum(a, b) {
  return a + b;
}
sum(1);
```

```expect
(none)
```

### case: a built-in with too few

```js
Object.is(value);
```

```expect
1:1 js/no-invalid-argument-count
```

### case: a built-in with none

```js
Promise.allSettled();
```

```expect
1:1 js/no-invalid-argument-count
```

### case: Math.random with an argument

```js
const n = Math.random(seed);
```

```expect
1:11 js/no-invalid-argument-count
```

### case: a constructor with too many

```js
const s = new Set(items, extra);
```

```expect
1:11 js/no-invalid-argument-count
```

### case: valid calls

```js
Object.is(a, b);
Math.random();
new Set(items);
parseInt(text, 10);
JSON.stringify(v, null, 2);
```

```expect
(none)
```

### case: a rest parameter takes any number

```js
function join(first, ...rest) {
  return [first, ...rest];
}
join(1, 2, 3);
```

```expect
(none)
```

### case: a default still counts as a parameter

```js
function f(a, b = 1) {
  return a + b;
}
f(1, 2);
```

```expect
(none)
```

### case: a function that reads arguments

```js
function f(a) {
  return arguments.length;
}
f(1, 2, 3);
```

```expect
(none)
```

### case: a spread call

```js
const f = (a) => a;
f(...items);
new Set(value, extra, ...values);
```

```expect
3:1 js/no-invalid-argument-count
```

### case: a shadowed global

```js
const Object = { is: (a) => a };
Object.is(1);
```

```expect
(none)
```

### case: a reassigned function

```js
let f = (a) => a;
f = (a, b) => a + b;
f(1, 2);
```

```expect
(none)
```

### case: an immediately invoked function

```js
(function (a) {
  return a;
})(1, 2);
```

```expect
1:1 js/no-invalid-argument-count
```

### case: a configured member path

```js options={"patterns":{"canvas.ctx.drawImage":[3,5,9]}}
canvas.ctx.drawImage(image, dx, dy);
```

```expect
(none)
```

### case: a configured exact pattern

```js options={"patterns":{"drawImage":[3,5,9]}}
drawImage(image, dx);
```

```expect
1:1 js/no-invalid-argument-count
```
