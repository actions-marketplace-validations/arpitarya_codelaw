# js/no-unsafe-json-serialization — cases

### case: a Map at the top

```js
JSON.stringify(new Map(entries));
```

```expect
1:16 js/no-unsafe-json-serialization
```

```fixed
JSON.stringify(Object.fromEntries(new Map(entries)));
```

### case: a BigInt in an array

```js
JSON.stringify([1n]);
```

```expect
1:17 js/no-unsafe-json-serialization
```

```fixed
JSON.stringify([String(1n)]);
```

### case: inside a call it stays

```js
JSON.stringify(wrap(new Set(xs)));
```

```expect
(none)
```
