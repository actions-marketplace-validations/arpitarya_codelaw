# js/max-nested-calls — cases

### case: a chain is not nesting

```js
query().filter().map().toArray();
```

```expect
(none)
```

### case: a callback starts again

```js
a(b(c(() => d(e(f())))));
```

```expect
(none)
```
