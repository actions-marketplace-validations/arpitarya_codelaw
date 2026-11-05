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

### case: a bitwise or in a condition becomes a logical or

```js
while (a | b) run();
```

```expect
1:8 js/no-bitwise
```

```fixed
while (a || b) run();
```

### case: a bitwise and outside a condition carries no edit

```js
const m = a & b;
```

```expect
1:11 js/no-bitwise
```
