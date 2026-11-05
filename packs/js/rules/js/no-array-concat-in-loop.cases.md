# js/no-array-concat-in-loop — cases

### case: one array argument is rewritten to a push

```js
for (const page of pages) {
  rows = rows.concat(page.rows);
}
```

```expect
2:3 js/no-array-concat-in-loop
```

```fixed
for (const page of pages) {
  rows.push(...page.rows);
}
```

### case: several arguments carry no edit

```js
for (const page of pages) {
  rows = rows.concat(page.a, page.b);
}
```

```expect
2:3 js/no-array-concat-in-loop
```
