# js/prefer-string-raw — cases

### case: a string with another escape stays

```js
const s = "a\\b\n";
```

```expect
(none)
```

### case: a string ending in a backslash stays

```js
const s = "dir\\";
```

```expect
(none)
```
