# js/use-top-level-regex — cases

### case: in an arrow function

```js
const check = (v) => /a+/i.test(v);
```

```expect
1:22 js/use-top-level-regex
```

### case: a stateful regex

```js
function next(s) {
  return /a/g.exec(s);
}
```

```expect
(none)
```

### case: at the top level

```js
const A = /a+/;
```

```expect
(none)
```
