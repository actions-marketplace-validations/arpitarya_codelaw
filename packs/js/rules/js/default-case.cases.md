# js/default-case — cases

### case: an empty switch gets a default

```js
switch (x) {
  case 1:
    a();
    break;
}
```

```expect
1:1 js/default-case
```

```fixed
switch (x) {
  case 1:
    a();
    break;
  default:
    break;
}
```
