# js/no-self-compare — cases

### case: equality with itself is the negated NaN test

```js
if (x === x) run();
```

```expect
1:5 js/no-self-compare
```

```fixed
if (!Number.isNaN(x)) run();
```

### case: a member operand carries no edit

```js
if (a.b !== a.b) run();
```

```expect
1:5 js/no-self-compare
```
