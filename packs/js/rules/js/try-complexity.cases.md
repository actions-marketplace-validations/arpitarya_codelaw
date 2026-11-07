# js/try-complexity — cases

### case: a conditional expression

```js
try {
  const value = a ? b() : c();
} catch (error) {
  report(error);
}
```

```expect
1:1 js/try-complexity
```

### case: a callback's branches don't count

```js
try {
  run(() => { if (x) y(); });
} catch (error) {
  report(error);
}
```

```expect
(none)
```
