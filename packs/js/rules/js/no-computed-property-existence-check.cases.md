# js/no-computed-property-existence-check — cases

### case: the in operator

```js
const known = name in handlers;
```

```expect
1:15 js/no-computed-property-existence-check
```

```fixed
const known = Object.hasOwn(handlers, name);
```

### case: a literal key stays

```js
if (handlers["a"]) {}
const k = "a" in handlers;
```

```expect
(none)
```

### case: a read used as a value stays

```js
const handler = handlers[name];
```

```expect
(none)
```
