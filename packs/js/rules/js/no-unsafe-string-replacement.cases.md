# js/no-unsafe-string-replacement — cases

### case: a variable

```js
const out = text.replaceAll("{x}", value);
```

```expect
1:13 js/no-unsafe-string-replacement
```

```fixed
const out = text.replaceAll("{x}", () => value);
```

### case: a literal stays

```js
const out = text.replace("{x}", "y");
```

```expect
(none)
```

### case: a function stays

```js
const out = text.replace("{x}", () => y);
```

```expect
(none)
```
