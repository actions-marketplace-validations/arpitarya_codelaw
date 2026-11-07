### case: a last clause that falls out of the switch is moved by hand

```js
switch (x) {
  default:
    a();
    break;
  case 1:
    b();
}
```

```expect
2:3 js/default-case-last
```

### case: default first, between two cases

```js
switch (x) {
  default:
    a();
    break;
  case 1:
    b();
    break;
  case 2:
    c();
    return;
}
```

```expect
2:3 js/default-case-last
```

```fixed
switch (x) {
  case 1:
    b();
    break;
  case 2:
    c();
    return;
  default:
    a();
    break;
}
```
