# js/no-unreadable-for-of-expression — cases

### case: a conditional iterable

```js
for (const x of ready ? a : b) {
  use(x);
}
```

```expect
1:1 js/no-unreadable-for-of-expression
```

### case: Object.keys of a name stays

```js
for (const k of Object.keys(config)) {
  use(k);
}
```

```expect
(none)
```

### case: for…in stays

```js
for (const k in getObject(make(x))) {
  use(k);
}
```

```expect
(none)
```
