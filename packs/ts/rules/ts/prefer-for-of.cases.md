# ts/prefer-for-of — cases

### case: the index is used on its own

```ts
for (let i = 0; i < xs.length; i++) {
  print(i, xs[i]);
}
```

```expect
```

### case: the element is assigned

```ts
for (let i = 0; i < xs.length; i++) {
  xs[i] = 0;
}
```

```expect
```

### case: another array is indexed

```ts
for (let i = 0; i < xs.length; i++) {
  use(ys[i]);
}
```

```expect
```
