# js/prefer-single-replace — cases

### case: different replacements

```js
s.replaceAll("a", "1").replaceAll("b", "2");
```

```expect
(none)
```

### case: replace with a string searches once

```js
s.replace("x", "_").replace("y", "_");
```

```expect
(none)
```
