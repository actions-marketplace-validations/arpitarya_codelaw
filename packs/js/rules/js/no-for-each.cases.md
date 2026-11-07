### case: a callback that returns is rewritten by hand

```js
items.forEach((item) => {
  if (!item) return;
  send(item);
});
```

```expect
1:1 js/no-for-each
```

### case: a parameter without parentheses

```js
items.forEach(item => {
  send(item);
});
```

```expect
1:1 js/no-for-each
```

```fixed
for (const item of items) {
  send(item);
}
```
