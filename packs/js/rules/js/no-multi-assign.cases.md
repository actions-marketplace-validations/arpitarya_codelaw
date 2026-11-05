# js/no-multi-assign — cases

### case: a chain of three splits right to left

```js
a = b = c = 0;
```

```expect
1:1 js/no-multi-assign
1:5 js/no-multi-assign
```

```fixed
c = 0;
b = c;
a = b;
```

### case: a member target carries no edit

```js
a.x = b = 0;
```

```expect
1:1 js/no-multi-assign
```
