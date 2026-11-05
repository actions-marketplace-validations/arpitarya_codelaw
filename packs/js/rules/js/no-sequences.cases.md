# js/no-sequences — cases

### case: a for header is allowed

```js
for (i = 0, j = 9; i < j; i++, j--) step(i, j);
```

```expect

```

### case: parentheses are allowed

```js
const x = (a(), b());
```

```expect

```

### case: a return value splits

```js
function f() {
  return a(), b();
}
```

```expect
2:10 js/no-sequences
```

```fixed
function f() {
  a(); return b();
}
```

### case: a chain of three carries no edit

```js
a(), b(), c();
```

```expect
1:1 js/no-sequences
```

### case: a sequence in a call argument carries no edit

```js
f((a(), b()), c);
x = g(a(), b());
```

```expect
(none)
```
