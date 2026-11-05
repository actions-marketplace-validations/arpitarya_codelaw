# js/no-accidental-bitwise-operator — cases

### case: a bitwise or in a while condition

```js
while (a | b) run();
```

```expect
1:8 js/no-accidental-bitwise-operator
```

```fixed
while (a || b) run();
```
