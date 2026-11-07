# js/no-break-in-nested-loop — cases

### case: a switch inside a loop

```js
for (const item of items) {
  switch (item.type) {
    case "a":
      break;
  }
}
```

```expect
4:7 js/no-break-in-nested-loop
```

### case: a labelled break stays

```js
outer: for (const row of rows) {
  for (const cell of row) {
    break outer;
  }
}
```

```expect
(none)
```

### case: a break of a single loop stays

```js
for (const item of items) {
  if (item.done) break;
}
```

```expect
(none)
```

### case: a function boundary stops the search

```js
for (const item of items) {
  item.run(() => {
    while (true) {
      break;
    }
  });
}
```

```expect
(none)
```
