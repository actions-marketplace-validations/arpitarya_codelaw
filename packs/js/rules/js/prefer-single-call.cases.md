### case: three pushes become one

```js
list.push(a);
list.push(b);
list.push(c, d);
```

```expect
1:1 js/prefer-single-call
2:1 js/prefer-single-call
```

```fixed
list.push(a, b, c, d);
```

### case: the same value pushed twice is pushed twice

```js
list.push(0);
list.push(0);
```

```expect
1:1 js/prefer-single-call
```

```fixed
list.push(0, 0);
```
