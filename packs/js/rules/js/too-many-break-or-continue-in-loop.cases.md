# js/too-many-break-or-continue-in-loop — cases

### case: breaks of a switch don't count

```js
for (const item of items) {
  switch (item.kind) {
    case "a":
      a();
      break;
    case "b":
      b();
      break;
  }
  if (item.last) break;
}
```

```expect
(none)
```
