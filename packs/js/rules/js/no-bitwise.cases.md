# js/no-bitwise — cases

### case: logical operators are fine

```js
if (a && b || c) run();
x ||= 1;
y &&= 2;
```

```expect

```

### case: a bitwise assignment

```js
flags |= 4;
```

```expect
1:1 js/no-bitwise
```

### case: bitwise not

```js
const m = ~mask;
```

```expect
1:11 js/no-bitwise
```
