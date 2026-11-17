# js/consistent-function-style — cases

### case: silent without the option

```js
const add = (a, b) => a + b;
items.map(function (x) {
  return x;
});
```

```expect
(none)
```

### case: a named function as a declaration

```js options={"namedFunctions":"declaration"}
const add = (a, b) => a + b;
function sub(a, b) {
  return a - b;
}
```

```expect
1:13 js/consistent-function-style
```

### case: a callback as an arrow, with a suggestion

```js options={"callbacks":"arrow-function"}
items.map(function (x) {
  return x * 2;
});
```

```expect
1:11 js/consistent-function-style
```

### case: a callback that uses this has no suggestion

```js options={"callbacks":"arrow-function"}
items.forEach(function (x) {
  this.add(x);
});
```

```expect
1:15 js/consistent-function-style
```

### case: a callback as a function expression

```js options={"callbacks":"function-expression"}
items.map((x) => x * 2);
```

```expect
1:11 js/consistent-function-style
```

### case: object properties as methods

```js options={"objectProperties":"method"}
const api = {
  run: () => 1,
  stop() {},
};
```

```expect
2:8 js/consistent-function-style
```

### case: a reassigned variable

```js options={"reassignedVariables":"function-expression"}
let handler = () => 1;
handler = () => 2;
```

```expect
1:15 js/consistent-function-style
2:11 js/consistent-function-style
```

### case: a default export is read only when it is set

```js options={"namedFunctions":"arrow-function"}
export default function () {}
```

```expect
(none)
```

### case: a generator is never asked to be an arrow

```js options={"namedFunctions":"arrow-function"}
const gen = function* () {};
```

```expect
(none)
```

### case: a function called on the spot

```js options={"default":"arrow-function"}
(function () {})();
```

```expect
(none)
```
