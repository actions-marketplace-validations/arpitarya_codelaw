# js/guard-for-in — cases

### case: for-of is not for-in

```js
for (const x of xs) {
  use(x);
}
```

```expect

```

### case: a filtered for-in

```js
for (const k in o) {
  if (!Object.hasOwn(o, k)) continue;
  use(k);
}
```

```expect

```
