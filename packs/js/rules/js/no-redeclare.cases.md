# js/no-redeclare — cases

### case: a bare redeclaration is dropped

```js
var count = 1;
var count;
use(count);
```

```expect
2:5 js/no-redeclare
```

```fixed
var count = 1;
use(count);
```

### case: a second declarator carries no edit

```js
var count = 1;
var other = 0, count = 2;
```

```expect
2:16 js/no-redeclare
```
