# js/prefer-array-from-async — cases

### case: a loop that transforms stays

```js
const rows = [];
for await (const row of cursor) {
  rows.push(parse(row));
}
```

```expect
(none)
```

### case: a statement in between stays

```js
const rows = [];
log();
for await (const row of cursor) {
  rows.push(row);
}
```

```expect
(none)
```
