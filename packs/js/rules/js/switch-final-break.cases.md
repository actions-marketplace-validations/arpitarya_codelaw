# js/switch-final-break — cases

### case: a break before another statement is not final

```js
switch (x) {
  case 1:
    if (a) break;
    go();
}
```

```expect

```

### case: a labelled break is left alone

```js
outer: for (;;) {
  switch (x) {
    case 1:
      break outer;
  }
}
```

```expect

```
