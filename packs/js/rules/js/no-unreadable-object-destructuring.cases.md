# js/no-unreadable-object-destructuring — cases

### case: a nested array

```js
const { list: [first] } = state;
```

```expect
1:9 js/no-unreadable-object-destructuring
```

### case: a computed key beside a rest stays

```js
const { [key]: drop, ...rest } = state;
```

```expect
(none)
```

### case: a nested object stays

```js
const { a: { b } } = state;
```

```expect
(none)
```
