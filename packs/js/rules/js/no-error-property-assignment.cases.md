# js/no-error-property-assignment — cases

### case: an assignment right after the declaration

```js
const error = new Error("failed");
error.stack = saved;
```

```expect
2:1 js/no-error-property-assignment
```

### case: message stays

```js
const error = new Error("failed");
error.message = text;
```

```expect
(none)
```

### case: an unknown value stays

```js
Object.assign(error, { name });
```

```expect
(none)
```
