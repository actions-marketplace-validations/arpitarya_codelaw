# js/no-top-level-assignment-in-function — cases

### case: assigned at the top level

```js
let c;
c = 1;
export { c };
```

```expect
(none)
```

### case: a property of a module-level object

```js
const s = {};
function f() {
  s.c = 1;
}
f();
```

```expect
(none)
```

### case: an arrow that increments

```js
let i = 0;
const next = () => i++;
next();
```

```expect
2:20 js/no-top-level-assignment-in-function
```

### case: a parameter of the same name

```js
let c = 0;
function f(c) {
  c = 1;
  return c;
}
f(c);
```

```expect
(none)
```
