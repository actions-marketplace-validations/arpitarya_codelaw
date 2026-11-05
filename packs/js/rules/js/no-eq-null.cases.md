# js/no-eq-null — cases

### case: not-equal and the reversed operand order

```js
if (a != null) run();
if (null == b) run();
```

```expect
1:5 js/no-eq-null
2:5 js/no-eq-null
```

```fixed
if ((a !== null && a !== undefined)) run();
if ((b === null || b === undefined)) run();
```

### case: a member operand is outside the auto subset

```js
if (user.name == null) run();
```

```expect
1:5 js/no-eq-null
```
