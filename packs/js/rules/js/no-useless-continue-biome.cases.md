# js/no-useless-continue-biome — cases

### case: a continue in the middle is needed

```js
for (const x of xs) {
  if (x) continue;
  log(x);
}
```

```expect

```

### case: a labelled continue is not checked

```js
outer: for (const x of xs) {
  for (const y of ys) {
    log(y);
    continue outer;
  }
}
```

```expect

```
