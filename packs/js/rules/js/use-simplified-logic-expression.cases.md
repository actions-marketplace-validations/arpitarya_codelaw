# js/use-simplified-logic-expression — cases

### case: a literal true operand

```js
const v = true && check();
```

```expect
1:11 js/use-simplified-logic-expression
```

```fixed
const v = check();
```

### case: two negations with and

```js
if (!a && !b) run();
```

```expect
1:5 js/use-simplified-logic-expression
```

```fixed
if (!(a || b)) run();
```

### case: a trailing false

```js
const v = value || false;
```

```expect
(none)
```
