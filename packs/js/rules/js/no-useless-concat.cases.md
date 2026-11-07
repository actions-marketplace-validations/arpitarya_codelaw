### case: three strings join in two passes

```js
const s = "a" + "b" + "c";
```

```expect
1:11 js/no-useless-concat
```

```fixed
const s = "abc";
```

### case: quotes that differ are left as they are

```js
const s = "a" + 'b';
```

```expect
1:11 js/no-useless-concat
```


### case: a numeric escape before the join is left as it is

```js
const s = '\0' + '1';
```

```expect
1:11 js/no-useless-concat
```

