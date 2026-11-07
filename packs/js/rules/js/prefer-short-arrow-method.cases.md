# js/prefer-short-arrow-method — cases

### case: an object result is wrapped

```js
const f = { make() { return { id: 1 }; } };
```

```expect
1:13 js/prefer-short-arrow-method
```

```fixed
const f = { make: () => ({ id: 1 }) };
```

### case: an async method

```js
const f = { async load(id) { return fetch(id); } };
```

```expect
1:13 js/prefer-short-arrow-method
```

```fixed
const f = { load: async (id) => fetch(id) };
```

### case: a method using this stays

```js
const f = { name() { return this.id; } };
```

```expect
(none)
```

### case: a getter stays

```js
const f = { get name() { return 1; } };
```

```expect
(none)
```

### case: a class method stays

```js
class A { name() { return 1; } }
```

```expect
(none)
```

### case: two statements stay

```js
const f = { name() { log(); return 1; } };
```

```expect
(none)
```
