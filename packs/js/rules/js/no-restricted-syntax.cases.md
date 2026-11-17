# js/no-restricted-syntax — cases

### case: silent without the option

```js
for (const key in object) {
  use(key);
}
const merged = Object.assign({}, a);
```

```expect
(none)
```

### case: a pattern with the default message

```js options={"patterns":["Object.assign($$$ARGS)"]}
const merged = Object.assign({}, base);
```

```expect
1:16 js/no-restricted-syntax
```

### case: a kind

```js options={"kinds":["with_statement"]}
with (scope) {
  run();
}
use(scope);
```

```expect
1:1 js/no-restricted-syntax
```

### case: a pattern with a statement

```js options={"patterns":["with\u0020($O)\u0020$B"]}
with (scope) {
  run();
}
```

```expect
1:1 js/no-restricted-syntax
```

### case: a pattern that does not parse is reported once

```js options={"patterns":["a;\u0020b"]}
const a = 1;
const b = 2;
```

```expect
1:1 js/no-restricted-syntax
```
