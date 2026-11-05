# js/use-isnan — cases

### case: each operator and side

```js
a !== NaN;
NaN == b;
NaN != c;
```

```expect
1:1 js/use-isnan
2:1 js/use-isnan
3:1 js/use-isnan
```

```fixed
!Number.isNaN(a);
Number.isNaN(b);
!Number.isNaN(c);
```

### case: an ordering comparison has no edit

```js
d < NaN;
```

```expect
1:1 js/use-isnan
```
