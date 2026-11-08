# js/prefer-get-or-insert-computed — cases

### case: already a callback

```js
m.getOrInsert(k, () => make());
```

```expect
(none)
```

### case: an object literal value is wrapped in parentheses

```js
m.getOrInsert(k, { at: now() });
```

```expect
1:18 js/prefer-get-or-insert-computed
```

```fixed
m.getOrInsertComputed(k, () => ({ at: now() }));
```
