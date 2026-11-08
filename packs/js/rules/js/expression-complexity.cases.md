# js/expression-complexity — cases

### case: ternaries count

```js
const v = a ? b : c ? d : e || f && g;
```

```expect
1:11 js/expression-complexity
```

### case: three stay

```js
if (a && b && c && d) {}
```

```expect
(none)
```
