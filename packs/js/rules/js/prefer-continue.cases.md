# js/prefer-continue — cases

### case: one statement

```js
while (n--) {
  if (n) {
    a();
  }
}
```

```expect
(none)
```

### case: ends in an exit

```js
for (;;) {
  if (c) {
    a();
    return;
  }
}
```

```expect
(none)
```
