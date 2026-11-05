# js/no-continue — cases

### case: a guard with several statements after it

```js
for (const x of xs) {
  if (!x) continue;
  a(x);
  b(x);
}
```

```expect
2:11 js/no-continue
```

```fixed
for (const x of xs) {
  if (!(!x)) {
    a(x);
  b(x);
  }
}
```

### case: a labelled continue carries no edit

```js
outer: for (const x of xs) {
  for (const y of x) {
    if (!y) continue outer;
    use(y);
  }
}
```

```expect
3:13 js/no-continue
```
