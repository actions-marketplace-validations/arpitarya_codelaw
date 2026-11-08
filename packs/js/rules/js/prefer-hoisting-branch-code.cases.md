# js/prefer-hoisting-branch-code — cases

### case: no final else

```js
if (a) {
  x();
} else if (b) {
  x();
}
```

```expect
(none)
```

### case: the whole branch

```js
if (a) {
  x();
} else {
  x();
}
```

```expect
(none)
```
