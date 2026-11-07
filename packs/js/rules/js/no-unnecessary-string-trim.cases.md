# js/no-unnecessary-string-trim — cases

### case: endsWith

```js
const done = line.trim().endsWith(";");
```

```expect
1:14 js/no-unnecessary-string-trim
```

```fixed
const done = line.trimEnd().endsWith(";");
```

### case: a search value ending in a space stays

```js
const ok = line.trim().startsWith("foo ");
```

```expect
(none)
```

### case: a position argument stays

```js
const ok = line.trim().startsWith("-", 1);
```

```expect
(none)
```

### case: a variable search value stays

```js
const ok = line.trim().startsWith(prefix);
```

```expect
(none)
```
